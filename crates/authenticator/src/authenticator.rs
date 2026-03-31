use async_trait::async_trait;

use std::collections::HashSet;

use crate::error::AuthenticatorError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserToken {
    pub id: Uuid,
    pub groups: HashSet<Uuid>,
    pub roles: HashSet<Uuid>,
}

#[async_trait]
pub trait Authenticator: Send + Sync {
    async fn validate(&self, token: &str) -> Result<UserToken, Box<AuthenticatorError>>;
}
