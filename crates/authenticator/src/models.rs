//! Types shared between the [`Authenticator`](crate::Authenticator) trait and its
//! backends: the resolved caller identity, the OAuth flow's inputs and outputs,
//! and the identity claims returned by the provider's userinfo endpoint.

use std::time::Duration;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The caller resolved from a credential (a provider JWT or an API key).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserToken {
    pub id: Uuid,
    pub realm: String,
}

/// Which provider page the browser should land on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoginScreen {
    Login,
    Register,
}

/// Tokens issued by the provider after a code exchange or a refresh.
#[derive(Debug, Clone)]
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub access_expires_in: Option<Duration>,
}

/// The result of a successful code exchange: the tokens plus the post-login
/// redirect that was stored when the flow started.
#[derive(Debug, Clone)]
pub struct AuthSession {
    pub tokens: AuthTokens,
    pub redirect: Option<String>,
}

/// Identity claims returned by the provider's userinfo endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct UserInfo {
    pub sub: Uuid,
    #[serde(default)]
    pub preferred_username: String,
    #[serde(default)]
    pub given_name: String,
    #[serde(default)]
    pub family_name: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub email_verified: bool,
}
