//! API-key credentials: a sha256 hex digest looked up in the database, memoised
//! in the shared cache for 300s.

use std::sync::Arc;

use cache::Cache;
use database::Database;
use sha2::{Digest, Sha256};
use tokio::sync::RwLock;

use crate::error::AuthenticatorError;
use crate::models::Caller;

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
    ) -> Result<Caller, Box<AuthenticatorError>> {
        let hashed = hex_sha256(token);

        if let Some(caller) = self.try_get_cache(&hashed).await {
            return Ok(caller);
        }

        let api_key = self
            .database
            .read()
            .await
            .read_api_key_by_hash(&hashed)
            .await
            .map_err(|_| AuthenticatorError::AuthenticationFailure)?;

        let caller = Caller {
            id: api_key.owner,
            realm: "api_key".to_string(),
        };

        self.set_cache(&caller, hashed.as_str()).await;

        Ok(caller)
    }

    /// Removes the cache entry of one key, leaving every other entry in place.
    pub(super) async fn forget(
        &self,
        hashed: &str,
    ) -> Result<(), Box<AuthenticatorError>> {
        self.cache
            .read()
            .await
            .delete(hashed)
            .await
            .map_err(|error| Box::new(AuthenticatorError::from(error)))
    }

    /// Gets the token from local cache
    async fn try_get_cache(&self, hashed: &str) -> Option<Caller> {
        if let Some(value) = self.cache.read().await.get_nofail(hashed).await
            && let Ok(caller) = serde_json::from_value::<Caller>(value)
        {
            return Some(caller);
        }
        None
    }

    /// Sets the token into local cache
    async fn set_cache(&self, caller: &Caller, hashed: &str) {
        if let Ok(value) = serde_json::to_value(caller) {
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

test_utils::tests_file!("_tests/test_api_key.rs");
