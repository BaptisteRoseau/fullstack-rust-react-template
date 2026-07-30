use async_trait::async_trait;

use crate::error::AuthenticatorError;
use crate::models::{AuthSession, AuthTokens, LoginScreen, UserInfo, UserToken};

/// Everything the application needs from an identity provider: validating the
/// credentials callers present, and driving the browser login flow on their behalf.
#[async_trait]
pub trait Authenticator: Send + Sync {
    /// Resolves a caller-supplied credential — a provider JWT or an API key —
    /// into the user it identifies.
    async fn validate(&self, token: &str) -> Result<UserToken, Box<AuthenticatorError>>;

    /// Builds the provider URL the browser must be sent to, persisting the PKCE
    /// verifier and the post-login `redirect` under a freshly generated CSRF state.
    async fn authorize_url(
        &self,
        screen: LoginScreen,
        redirect: Option<&str>,
    ) -> Result<String, Box<AuthenticatorError>>;

    /// Exchanges an authorization code for a session, validating the CSRF state
    /// and consuming it so it cannot be replayed.
    async fn exchange_code(
        &self,
        code: &str,
        state: &str,
    ) -> Result<AuthSession, Box<AuthenticatorError>>;

    /// Exchanges a refresh token for a fresh token pair.
    async fn refresh_tokens(
        &self,
        refresh_token: &str,
    ) -> Result<AuthTokens, Box<AuthenticatorError>>;

    /// Fetches the identity claims backing the current-user endpoint.
    async fn userinfo(
        &self,
        access_token: &str,
    ) -> Result<UserInfo, Box<AuthenticatorError>>;

    /// Revokes the provider-side session.
    async fn logout(&self, refresh_token: &str) -> Result<(), Box<AuthenticatorError>>;
}
