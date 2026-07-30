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
            id: api_key.owner(),
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
mod tests {
    use super::*;
    use cache::backends::hash_map::HashMapCache;
    use database::models::ApiKey;
    use database::testing::MockDatabase;
    use std::collections::HashMap;
    use uuid::Uuid;

    fn make_validator(
        cache: Arc<RwLock<dyn Cache>>,
        database: Arc<RwLock<dyn Database>>,
    ) -> ApiKeyValidator {
        ApiKeyValidator::new(cache, database)
    }

    fn make_api_key(hash: &str, owner: Uuid) -> ApiKey {
        let now = chrono::Utc::now();
        ApiKey {
            id: Uuid::new_v4(),
            hash: hash.to_string(),
            name: "test-key".to_string(),
            owner,
            permissions: serde_json::json!({}),
            created_at: now,
            updated_at: now,
        }
    }

    #[test]
    fn hex_sha256_matches_known_vectors() {
        let empty = hex_sha256("");
        assert_eq!(
            empty, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            "sha256 of empty string mismatch, got={empty}"
        );

        let hello = hex_sha256("hello");
        assert_eq!(
            hello, "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
            "sha256 of \"hello\" mismatch, got={hello}"
        );
    }

    #[tokio::test]
    async fn set_cache_then_try_get_cache_round_trips() {
        let cache: Arc<RwLock<dyn Cache>> =
            Arc::new(RwLock::new(HashMapCache::default()));
        let database: Arc<RwLock<dyn Database>> =
            Arc::new(RwLock::new(MockDatabase::default()));
        let validator = make_validator(cache, database);

        let hashed = hex_sha256("my-secret-token");
        let token = UserToken {
            id: Uuid::new_v4(),
            realm: "api_key".to_string(),
        };

        assert!(
            validator.try_get_cache(&hashed).await.is_none(),
            "cache should be empty before set, hashed={hashed}"
        );

        validator.set_cache(&token, &hashed).await;

        let cached = validator
            .try_get_cache(&hashed)
            .await
            .expect("token should be present after set_cache");
        assert_eq!(
            cached.id, token.id,
            "cached id mismatch: got={}, want={}",
            cached.id, token.id
        );
        assert_eq!(
            cached.realm, token.realm,
            "cached realm mismatch: got={}, want={}",
            cached.realm, token.realm
        );
    }

    #[tokio::test]
    async fn try_get_cache_returns_none_on_miss() {
        let cache: Arc<RwLock<dyn Cache>> =
            Arc::new(RwLock::new(HashMapCache::default()));
        let database: Arc<RwLock<dyn Database>> =
            Arc::new(RwLock::new(MockDatabase::default()));
        let validator = make_validator(cache, database);

        assert!(
            validator.try_get_cache("unknown-hash").await.is_none(),
            "missing key should yield None"
        );
    }

    #[tokio::test]
    async fn validate_api_key_reads_from_database_and_caches() {
        let token = "raw-api-key-value";
        let hashed = hex_sha256(token);
        let owner = Uuid::new_v4();

        let mut api_keys_by_hash = HashMap::new();
        api_keys_by_hash.insert(hashed.clone(), make_api_key(&hashed, owner));

        let cache: Arc<RwLock<dyn Cache>> =
            Arc::new(RwLock::new(HashMapCache::default()));
        let database: Arc<RwLock<dyn Database>> = Arc::new(RwLock::new(MockDatabase {
            api_keys_by_hash,
            ..Default::default()
        }));
        let validator = make_validator(cache, database);

        let user_token = validator
            .validate(token)
            .await
            .expect("valid api key should resolve to a user token");
        assert_eq!(
            user_token.id, owner,
            "user id should be the api key owner: got={}, want={owner}",
            user_token.id
        );
        assert_eq!(
            user_token.realm, "api_key",
            "api key realm should be \"api_key\", got={}",
            user_token.realm
        );

        let cached = validator
            .try_get_cache(&hashed)
            .await
            .expect("validate should populate the cache");
        assert_eq!(
            cached.id, owner,
            "cached id should match owner: got={}, want={owner}",
            cached.id
        );
    }

    #[tokio::test]
    async fn validate_api_key_uses_cache_before_database() {
        let token = "cached-api-key";
        let hashed = hex_sha256(token);
        let owner = Uuid::new_v4();

        let cache: Arc<RwLock<dyn Cache>> =
            Arc::new(RwLock::new(HashMapCache::default()));
        // Empty database: a cache miss would surface as AuthenticationFailure.
        let database: Arc<RwLock<dyn Database>> =
            Arc::new(RwLock::new(MockDatabase::default()));
        let validator = make_validator(cache, database);

        validator
            .set_cache(
                &UserToken {
                    id: owner,
                    realm: "api_key".to_string(),
                },
                &hashed,
            )
            .await;

        let user_token = validator
            .validate(token)
            .await
            .expect("cached api key should resolve without database");
        assert_eq!(
            user_token.id, owner,
            "cached user id mismatch: got={}, want={owner}",
            user_token.id
        );
    }

    #[tokio::test]
    async fn validate_api_key_fails_when_unknown() {
        let cache: Arc<RwLock<dyn Cache>> =
            Arc::new(RwLock::new(HashMapCache::default()));
        let database: Arc<RwLock<dyn Database>> =
            Arc::new(RwLock::new(MockDatabase::default()));
        let validator = make_validator(cache, database);

        let result = validator.validate("does-not-exist").await;
        let is_auth_failure = matches!(
            result.as_ref().map_err(|e| e.as_ref()),
            Err(AuthenticatorError::AuthenticationFailure)
        );
        assert!(
            is_auth_failure,
            "unknown api key should fail authentication, got={result:?}"
        );
    }
}
