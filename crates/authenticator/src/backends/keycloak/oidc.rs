//! Authorization Code + PKCE flow against Keycloak.
//!
//! Drives the OAuth Backend-for-Frontend flow: building the authorize/registration
//! redirect, exchanging the authorization code, refreshing and revoking tokens,
//! and fetching userinfo claims. The PKCE verifier and the post-login redirect are
//! stashed in the shared cache (keyed by the CSRF state) between `authorize_url`
//! and `exchange_code`, which doubles as CSRF protection.

use std::sync::Arc;

use config::AuthenticatorConfig;
use oauth2::basic::{BasicClient, BasicTokenResponse};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EndpointNotSet,
    EndpointSet, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, RefreshToken, Scope,
    TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use cache::Cache;

use crate::error::{AuthenticatorError, oidc_error};
use crate::models::{AuthSession, AuthTokens, LoginScreen, UserInfo};

use super::endpoints::Endpoints;

/// `BasicClient` once the auth and token endpoints (and redirect URI) are set.
type ConfiguredClient =
    BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>;

const STATE_CACHE_PREFIX: &str = "oidc_state:";
const STATE_TTL_SECONDS: u32 = 600;
const SCOPES: [&str; 3] = ["openid", "email", "profile"];

/// Authorization Code + PKCE flow against Keycloak.
///
/// The HTTP client comes from `oauth2::reqwest` (its pinned reqwest version) so it
/// satisfies the crate's `AsyncHttpClient` bound; the rest of the workspace uses a
/// newer reqwest for unrelated calls.
pub(super) struct OidcFlow {
    client: ConfiguredClient,
    http: oauth2::reqwest::Client,
    cache: Arc<RwLock<dyn Cache>>,
    registrations_url: String,
    logout_url: String,
    userinfo_url: String,
    client_id: String,
    client_secret: String,
}

impl OidcFlow {
    /// Builds the flow from configuration and the derived Keycloak endpoints.
    pub(super) fn try_new(
        config: &AuthenticatorConfig,
        endpoints: &Endpoints,
        cache: Arc<RwLock<dyn Cache>>,
    ) -> Result<Self, Box<AuthenticatorError>> {
        let client = BasicClient::new(ClientId::new(config.client_id.clone()))
            .set_client_secret(ClientSecret::new(config.client_secret.clone()))
            .set_auth_uri(
                AuthUrl::new(endpoints.authorize.clone())
                    .map_err(|e| oidc_error(format!("invalid auth url: {e}")))?,
            )
            .set_token_uri(
                TokenUrl::new(endpoints.token.clone())
                    .map_err(|e| oidc_error(format!("invalid token url: {e}")))?,
            )
            .set_redirect_uri(
                RedirectUrl::new(config.redirect_url.clone())
                    .map_err(|e| oidc_error(format!("invalid redirect url: {e}")))?,
            );

        // The crate requires the HTTP client to refuse redirects to avoid SSRF.
        let http = oauth2::reqwest::Client::builder()
            .redirect(oauth2::reqwest::redirect::Policy::none())
            .build()
            .map_err(|e| oidc_error(format!("failed to build http client: {e}")))?;

        Ok(Self {
            client,
            http,
            cache,
            registrations_url: endpoints.registrations.clone(),
            logout_url: endpoints.logout.clone(),
            userinfo_url: endpoints.userinfo.clone(),
            client_id: config.client_id.clone(),
            client_secret: config.client_secret.clone(),
        })
    }

    /// Builds the Keycloak authorize (or registration) URL, persisting the PKCE
    /// verifier and post-login redirect under the generated CSRF state.
    pub(super) async fn authorize_url(
        &self,
        screen: LoginScreen,
        redirect: Option<&str>,
    ) -> Result<String, Box<AuthenticatorError>> {
        let (challenge, verifier) = PkceCodeChallenge::new_random_sha256();

        let mut request = self.client.authorize_url(CsrfToken::new_random);
        for scope in SCOPES {
            request = request.add_scope(Scope::new(scope.to_string()));
        }
        let (mut url, csrf) = request.set_pkce_challenge(challenge).url();

        if let LoginScreen::Register = screen {
            // `join` on an absolute URL just re-parses it, discarding `url` as a
            // base — this swaps host+path to the registrations endpoint while
            // letting us copy back the query string oauth2 built for us (scope,
            // state, PKCE challenge, redirect URI, …).
            let mut registration_url = url
                .join(&self.registrations_url)
                .map_err(|e| oidc_error(format!("invalid registrations url: {e}")))?;
            registration_url.set_query(url.query());
            url = registration_url;
        }

        let pending = PendingLogin {
            verifier: verifier.secret().clone(),
            redirect: redirect.map(str::to_string),
        };
        let value = serde_json::to_value(&pending)
            .map_err(|e| oidc_error(format!("serialize login state: {e}")))?;
        self.cache
            .read()
            .await
            .set(&state_key(csrf.secret()), &value, Some(STATE_TTL_SECONDS))
            .await
            .map_err(|e| oidc_error(format!("persist login state: {e}")))?;

        Ok(url.to_string())
    }

    /// Exchanges an authorization code for a session, validating the CSRF state
    /// and consuming it so it cannot be replayed.
    pub(super) async fn exchange_code(
        &self,
        code: &str,
        state: &str,
    ) -> Result<AuthSession, Box<AuthenticatorError>> {
        let key = state_key(state);
        let value = self
            .cache
            .read()
            .await
            .get(&key)
            .await
            .map_err(|e| oidc_error(format!("read login state: {e}")))?
            .ok_or_else(|| Box::new(AuthenticatorError::InvalidState))?;
        self.cache.read().await.delete_nofail(&key).await;

        let pending: PendingLogin = serde_json::from_value(value)
            .map_err(|e| oidc_error(format!("deserialize login state: {e}")))?;

        let token = self
            .client
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .set_pkce_verifier(PkceCodeVerifier::new(pending.verifier))
            .request_async(&self.http)
            .await
            .map_err(|e| {
                oidc_error(format!("authorization code exchange failed: {e}"))
            })?;

        Ok(AuthSession {
            tokens: tokens_from_response(&token),
            redirect: pending.redirect,
        })
    }

    /// Exchanges a refresh token for a fresh set of tokens.
    pub(super) async fn refresh_tokens(
        &self,
        refresh_token: &str,
    ) -> Result<AuthTokens, Box<AuthenticatorError>> {
        let token = self
            .client
            .exchange_refresh_token(&RefreshToken::new(refresh_token.to_string()))
            .request_async(&self.http)
            .await
            .map_err(|e| oidc_error(format!("token refresh failed: {e}")))?;

        Ok(tokens_from_response(&token))
    }

    /// Fetches the OIDC userinfo claims (sub, email, name, …) for an access token.
    pub(super) async fn userinfo(
        &self,
        access_token: &str,
    ) -> Result<UserInfo, Box<AuthenticatorError>> {
        let response = self
            .http
            .get(&self.userinfo_url)
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| oidc_error(format!("userinfo request failed: {e}")))?;

        let status = response.status();
        // The provider answers 401/403 for a revoked or otherwise refused token; that
        // is a client-side auth failure, not an internal error.
        if matches!(status.as_u16(), 401 | 403) {
            return Err(Box::new(AuthenticatorError::OidcRejected));
        }
        if !status.is_success() {
            return Err(oidc_error(format!("userinfo returned status {status}")));
        }

        response
            .json()
            .await
            .map_err(|e| oidc_error(format!("decode userinfo failed: {e}")))
    }

    /// Revokes the session at Keycloak's end-session endpoint.
    pub(super) async fn logout(
        &self,
        refresh_token: &str,
    ) -> Result<(), Box<AuthenticatorError>> {
        let response = self
            .http
            .post(&self.logout_url)
            .form(&[
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
                ("refresh_token", refresh_token),
            ])
            .send()
            .await
            .map_err(|e| oidc_error(format!("logout request failed: {e}")))?;

        let status = response.status();
        if !status.is_success() {
            return Err(oidc_error(format!("logout returned status {status}")));
        }
        Ok(())
    }
}

/// State persisted between `authorize_url` and `exchange_code`.
#[derive(Serialize, Deserialize)]
struct PendingLogin {
    verifier: String,
    redirect: Option<String>,
}

fn state_key(state: &str) -> String {
    format!("{STATE_CACHE_PREFIX}{state}")
}

fn tokens_from_response(token: &BasicTokenResponse) -> AuthTokens {
    AuthTokens {
        access_token: token.access_token().secret().clone(),
        refresh_token: token.refresh_token().map(|t| t.secret().clone()),
        access_expires_in: token.expires_in(),
    }
}
