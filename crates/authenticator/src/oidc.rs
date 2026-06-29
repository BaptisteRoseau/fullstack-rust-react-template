//! OAuth Backend-for-Frontend (BFF) client.
//!
//! Drives the Authorization Code + PKCE flow against Keycloak: building the
//! authorize/registration redirect, exchanging the authorization code, refreshing
//! and revoking tokens. The PKCE verifier and the post-login redirect are stashed
//! in the shared cache (keyed by the CSRF state) between `/auth/login` and
//! `/auth/callback`, which doubles as CSRF protection.

use crate::error::AuthenticatorError;
use cache::Cache;
use config::Config;
use oauth2::basic::{BasicClient, BasicTokenResponse};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EndpointNotSet,
    EndpointSet, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, RefreshToken, Scope,
    TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

/// `BasicClient` once the auth and token endpoints (and redirect URI) are set.
type ConfiguredClient =
    BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>;

const STATE_CACHE_PREFIX: &str = "oidc_state:";
const STATE_TTL_SECONDS: u32 = 600;
const AUTH_PATH: &str = "/protocol/openid-connect/auth";
const REGISTRATIONS_PATH: &str = "/protocol/openid-connect/registrations";

/// Which Keycloak page the browser should land on.
#[derive(Debug, Clone, Copy)]
pub enum LoginScreen {
    Login,
    Register,
}

/// Tokens returned by Keycloak after a code exchange or refresh.
pub struct OidcTokens {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub access_expires_in: Option<Duration>,
}

/// State persisted between `/auth/login` and `/auth/callback`.
#[derive(Serialize, Deserialize)]
struct PendingLogin {
    verifier: String,
    redirect: Option<String>,
}

/// OAuth BFF client wrapping the `oauth2` crate and the shared cache.
///
/// The HTTP client comes from `oauth2::reqwest` (its pinned reqwest version) so it
/// satisfies the crate's `AsyncHttpClient` bound; the rest of the workspace uses a
/// newer reqwest for unrelated calls.
pub struct OidcClient {
    client: ConfiguredClient,
    http: oauth2::reqwest::Client,
    cache: Arc<RwLock<dyn Cache>>,
    logout_endpoint: String,
    userinfo_endpoint: String,
    client_id: String,
    client_secret: String,
    scopes: Vec<String>,
    frontend_url: String,
    cookie_secure: bool,
}

impl OidcClient {
    /// Builds the client from configuration, deriving Keycloak's well-known
    /// endpoints from the issuer URL.
    pub fn try_new(
        config: &Config,
        cache: Arc<RwLock<dyn Cache>>,
    ) -> Result<Self, Box<AuthenticatorError>> {
        let oidc = &config.oidc;
        let issuer = oidc.issuer_url.trim_end_matches('/');
        let auth_endpoint = format!("{issuer}{AUTH_PATH}");
        let token_endpoint = format!("{issuer}/protocol/openid-connect/token");
        let logout_endpoint = format!("{issuer}/protocol/openid-connect/logout");
        let userinfo_endpoint = format!("{issuer}/protocol/openid-connect/userinfo");

        let client = BasicClient::new(ClientId::new(oidc.client_id.clone()))
            .set_client_secret(ClientSecret::new(oidc.client_secret.clone()))
            .set_auth_uri(
                AuthUrl::new(auth_endpoint)
                    .map_err(|e| oidc_err(format!("invalid auth url: {e}")))?,
            )
            .set_token_uri(
                TokenUrl::new(token_endpoint)
                    .map_err(|e| oidc_err(format!("invalid token url: {e}")))?,
            )
            .set_redirect_uri(
                RedirectUrl::new(oidc.redirect_url.clone())
                    .map_err(|e| oidc_err(format!("invalid redirect url: {e}")))?,
            );

        // The crate requires the HTTP client to refuse redirects to avoid SSRF.
        let http = oauth2::reqwest::Client::builder()
            .redirect(oauth2::reqwest::redirect::Policy::none())
            .build()
            .map_err(|e| oidc_err(format!("failed to build http client: {e}")))?;

        Ok(Self {
            client,
            http,
            cache,
            logout_endpoint,
            userinfo_endpoint,
            client_id: oidc.client_id.clone(),
            client_secret: oidc.client_secret.clone(),
            scopes: vec![
                "openid".to_string(),
                "email".to_string(),
                "profile".to_string(),
            ],
            frontend_url: oidc.frontend_url.clone(),
            cookie_secure: oidc.cookie_secure,
        })
    }

    /// Default frontend URL to redirect the browser to after login.
    pub fn frontend_url(&self) -> &str {
        &self.frontend_url
    }

    /// Whether auth cookies should carry the `Secure` attribute.
    pub fn cookie_secure(&self) -> bool {
        self.cookie_secure
    }

    /// Builds the Keycloak authorize (or registration) URL, persisting the PKCE
    /// verifier and post-login redirect under the generated CSRF state.
    pub async fn authorize_url(
        &self,
        screen: LoginScreen,
        redirect: Option<String>,
    ) -> Result<String, Box<AuthenticatorError>> {
        let (challenge, verifier) = PkceCodeChallenge::new_random_sha256();

        let mut request = self.client.authorize_url(CsrfToken::new_random);
        for scope in &self.scopes {
            request = request.add_scope(Scope::new(scope.clone()));
        }
        let (mut url, csrf) = request.set_pkce_challenge(challenge).url();

        if let LoginScreen::Register = screen {
            let registrations_path = url.path().replace(AUTH_PATH, REGISTRATIONS_PATH);
            url.set_path(&registrations_path);
        }

        let pending = PendingLogin {
            verifier: verifier.secret().clone(),
            redirect,
        };
        let value = serde_json::to_value(&pending)
            .map_err(|e| oidc_err(format!("serialize login state: {e}")))?;
        self.cache
            .read()
            .await
            .set(&state_key(csrf.secret()), &value, Some(STATE_TTL_SECONDS))
            .await
            .map_err(|e| oidc_err(format!("persist login state: {e}")))?;

        Ok(url.to_string())
    }

    /// Exchanges an authorization code for tokens, validating the CSRF state and
    /// returning the post-login redirect that was stored at login time.
    pub async fn exchange_code(
        &self,
        code: String,
        state: String,
    ) -> Result<(OidcTokens, Option<String>), Box<AuthenticatorError>> {
        let key = state_key(&state);
        let value = self
            .cache
            .read()
            .await
            .get(&key)
            .await
            .map_err(|e| oidc_err(format!("read login state: {e}")))?
            .ok_or_else(|| Box::new(AuthenticatorError::InvalidState))?;
        self.cache.read().await.delete_nofail(&key).await;

        let pending: PendingLogin = serde_json::from_value(value)
            .map_err(|e| oidc_err(format!("deserialize login state: {e}")))?;

        let token = self
            .client
            .exchange_code(AuthorizationCode::new(code))
            .set_pkce_verifier(PkceCodeVerifier::new(pending.verifier))
            .request_async(&self.http)
            .await
            .map_err(|e| oidc_err(format!("authorization code exchange failed: {e}")))?;

        Ok((tokens_from_response(&token), pending.redirect))
    }

    /// Exchanges a refresh token for a fresh set of tokens.
    pub async fn refresh(
        &self,
        refresh_token: String,
    ) -> Result<OidcTokens, Box<AuthenticatorError>> {
        let token = self
            .client
            .exchange_refresh_token(&RefreshToken::new(refresh_token))
            .request_async(&self.http)
            .await
            .map_err(|e| oidc_err(format!("token refresh failed: {e}")))?;

        Ok(tokens_from_response(&token))
    }

    /// Fetches the OIDC userinfo claims (sub, email, name, …) for an access token.
    pub async fn userinfo(
        &self,
        access_token: &str,
    ) -> Result<serde_json::Value, Box<AuthenticatorError>> {
        let response = self
            .http
            .get(&self.userinfo_endpoint)
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| oidc_err(format!("userinfo request failed: {e}")))?;

        if !response.status().is_success() {
            return Err(oidc_err(format!(
                "userinfo returned status {}",
                response.status()
            )));
        }

        response
            .json()
            .await
            .map_err(|e| oidc_err(format!("decode userinfo failed: {e}")))
    }

    /// Revokes the session at Keycloak's end-session endpoint.
    pub async fn logout(
        &self,
        refresh_token: String,
    ) -> Result<(), Box<AuthenticatorError>> {
        self.http
            .post(&self.logout_endpoint)
            .form(&[
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
                ("refresh_token", refresh_token.as_str()),
            ])
            .send()
            .await
            .map_err(|e| oidc_err(format!("logout request failed: {e}")))?;
        Ok(())
    }
}

fn state_key(state: &str) -> String {
    format!("{STATE_CACHE_PREFIX}{state}")
}

fn oidc_err(message: String) -> Box<AuthenticatorError> {
    Box::new(AuthenticatorError::Oidc(message))
}

fn tokens_from_response(token: &BasicTokenResponse) -> OidcTokens {
    OidcTokens {
        access_token: token.access_token().secret().clone(),
        refresh_token: token.refresh_token().map(|t| t.secret().clone()),
        access_expires_in: token.expires_in(),
    }
}
