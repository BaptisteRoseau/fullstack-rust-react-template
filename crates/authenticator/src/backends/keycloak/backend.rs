use std::sync::Arc;

use async_trait::async_trait;
use cache::Cache;
use config::Config;
use database::Database;
use tokio::sync::RwLock;

use crate::authenticator::Authenticator;
use crate::error::AuthenticatorError;
use crate::models::{AuthSession, AuthTokens, LoginScreen, UserInfo, UserToken};

use super::api_key::ApiKeyValidator;
use super::endpoints::Endpoints;
use super::jwt::JwtValidator;
use super::oidc::OidcFlow;

/// Keycloak-backed [`Authenticator`]: JWKS/JWT and API-key validation for the
/// resource-server side, Authorization Code + PKCE for the login side.
pub struct Keycloak {
    jwt: JwtValidator,
    api_keys: ApiKeyValidator,
    oidc: OidcFlow,
}

impl Keycloak {
    pub async fn try_new(
        config: &Config,
        cache: Arc<RwLock<dyn Cache>>,
        database: Arc<RwLock<dyn Database>>,
    ) -> Result<Self, Box<AuthenticatorError>> {
        let endpoints = Endpoints::from_issuer(&config.authenticator.issuer_url);
        let jwt =
            JwtValidator::new(&endpoints.jwks, config.authenticator.audiences.clone())
                .await;
        let oidc =
            OidcFlow::try_new(&config.authenticator, &endpoints, Arc::clone(&cache))?;
        Ok(Self {
            jwt,
            api_keys: ApiKeyValidator::new(cache, database),
            oidc,
        })
    }
}

#[async_trait]
impl Authenticator for Keycloak {
    async fn validate(&self, token: &str) -> Result<UserToken, Box<AuthenticatorError>> {
        // Only a JWT contains dots; anything else is an API key.
        if token.contains('.') {
            self.jwt.validate(token).await
        } else {
            self.api_keys.validate(token).await
        }
    }

    async fn authorize_url(
        &self,
        screen: LoginScreen,
        redirect: Option<&str>,
    ) -> Result<String, Box<AuthenticatorError>> {
        self.oidc.authorize_url(screen, redirect).await
    }

    async fn exchange_code(
        &self,
        code: &str,
        state: &str,
    ) -> Result<AuthSession, Box<AuthenticatorError>> {
        self.oidc.exchange_code(code, state).await
    }

    async fn refresh_tokens(
        &self,
        refresh_token: &str,
    ) -> Result<AuthTokens, Box<AuthenticatorError>> {
        self.oidc.refresh_tokens(refresh_token).await
    }

    async fn userinfo(
        &self,
        access_token: &str,
    ) -> Result<UserInfo, Box<AuthenticatorError>> {
        self.oidc.userinfo(access_token).await
    }

    async fn logout(&self, refresh_token: &str) -> Result<(), Box<AuthenticatorError>> {
        self.oidc.logout(refresh_token).await
    }
}
