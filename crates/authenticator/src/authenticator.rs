use async_trait::async_trait;

use crate::error::AuthenticatorError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserToken {
    pub id: Uuid,
    pub realm: String,
}

#[async_trait]
pub trait Authenticator: Send + Sync {
    async fn validate(&self, token: &str) -> Result<UserToken, Box<AuthenticatorError>>;
    async fn refresh(&mut self) -> Result<(), Box<AuthenticatorError>>;
}
