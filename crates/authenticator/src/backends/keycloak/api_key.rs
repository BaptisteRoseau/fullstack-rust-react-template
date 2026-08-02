//! API-key credentials: a sha256 hex digest looked up in the database, memoised
//! in the shared cache for 300s.

use std::sync::Arc;

use cache::Cache;
use database::Database;
use sha2::{Digest, Sha256};
use tokio::sync::RwLock;

use crate::error::AuthenticatorError;
use crate::models::UserToken;

pub(super) struct ApiKeyValidator {
    cache: Arc<RwLock<dyn Cache>>,
    database: Arc<RwLock<dyn Database>>,
}

impl ApiKeyValidator {
    pub(super) fn new(
        cache: Arc<RwLock<dyn Cache>>,
        database: Arc<RwLock<dyn Database>>,
    ) -> Self {
        Self { cache, database }
    }

    pub(super) async fn validate(
        &self,
        token: &str,
    ) -> Result<UserToken, Box<AuthenticatorError>> {
        let hashed = hex_sha256(token);

        if let Some(user_token) = self.try_get_cache(&hashed).await {
            return Ok(user_token);
        }

        let api_key = self
            .database
            .read()
            .await
            .read_api_key_by_hash(&hashed)
            .await
            .map_err(|_| AuthenticatorError::AuthenticationFailure)?;

        let user_token = UserToken {
            id: api_key.owner,
            realm: "api_key".to_string(),
        };

        self.set_cache(&user_token, hashed.as_str()).await;

        Ok(user_token)
    }

    /// Gets the token from local cache
    async fn try_get_cache(&self, hashed: &str) -> Option<UserToken> {
        if let Some(value) = self.cache.read().await.get_nofail(hashed).await
            && let Ok(user_token) = serde_json::from_value::<UserToken>(value)
        {
            return Some(user_token);
        }
        None
    }

    /// Sets the token into local cache
    async fn set_cache(&self, user_token: &UserToken, hashed: &str) {
        if let Ok(value) = serde_json::to_value(user_token) {
            self.cache
                .read()
                .await
                .set_nofail(hashed, &value, Some(300))
                .await;
        }
    }
}

/// Digest used as the API-key lookup key; must match how keys are stored.
pub(super) fn hex_sha256(input: &str) -> String {
    Sha256::digest(input.as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

#[cfg(test)]
#[path = "_tests/test_api_key.rs"]
mod tests;
