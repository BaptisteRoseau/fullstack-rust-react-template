use crate::{Authenticator, UserToken, error::AuthenticatorError};
use async_trait::async_trait;
use cache::Cache;
use config::Config;
use database::Database;
use jsonwebtoken::jwk::JwkSet;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct Claims {
    sub: String,
    iss: String,
}

/// Keycloak sets iss to "http://<host>/realms/<realm-name>"
/// This function extract the last segment <realm-name>.
fn realm_from_iss(iss: &str) -> Option<String> {
    Some(iss.trim_end_matches('/').rsplit('/').next()?.to_string())
}

pub struct Keycloak {
    provider_url: String,
    audiences: Vec<String>,
    keys: Option<JwkSet>,
    cache: Arc<RwLock<dyn Cache>>,
    database: Arc<RwLock<dyn Database>>,
}

impl Keycloak {
    pub async fn try_new(
        config: &Config,
        cache: Arc<RwLock<dyn Cache>>,
        database: Arc<RwLock<dyn Database>>,
    ) -> Result<Self, Box<AuthenticatorError>> {
        let mut authenticator = Self {
            provider_url: config.authenticator.provider_url.clone(),
            audiences: config.authenticator.audiences.clone(),
            keys: None,
            cache,
            database,
        };
        authenticator.refresh().await?;
        Ok(authenticator)
    }

    async fn validate_jwt(
        &self,
        token: &str,
    ) -> Result<UserToken, Box<AuthenticatorError>> {
        let header = decode_header(token)?;
        let kid = header.kid.ok_or("No 'kid' in token header")?;
        let jwks = self.keys.as_ref().ok_or(AuthenticatorError::NoJwk)?;
        let jwk = jwks.find(&kid).ok_or("No matching key found in JWKS")?;
        let decoding_key = DecodingKey::from_jwk(jwk)?;

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&self.audiences);

        let claims = decode::<Claims>(token, &decoding_key, &validation)?.claims;
        let id = Uuid::parse_str(&claims.sub)
            .map_err(|e| AuthenticatorError::Message(format!("invalid sub UUID: {e}")))?;
        let realm = realm_from_iss(&claims.iss)
            .ok_or(AuthenticatorError::InvalidRealm(claims.iss))?;

        Ok(UserToken { id, realm })
    }

    async fn validate_api_key(
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

fn hex_sha256(input: &str) -> String {
    Sha256::digest(input.as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

#[async_trait]
impl Authenticator for Keycloak {
    async fn validate(&self, token: &str) -> Result<UserToken, Box<AuthenticatorError>> {
        // Only JWT contains dots
        if token.contains('.') {
            self.validate_jwt(token).await
        } else {
            self.validate_api_key(token).await
        }
    }

    async fn refresh(&mut self) -> Result<(), Box<AuthenticatorError>> {
        self.keys = Some(reqwest::get(&self.provider_url).await?.json().await?);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cache::error::CacheError;
    use database::models::ApiKey;
    use database::testing::MockDatabase;
    use serde_json::Value;
    use std::collections::HashMap;
    use tokio::sync::Mutex;

    /// In-memory cache backed by a `HashMap`, ignoring expiry timeouts.
    #[derive(Default)]
    struct MockCache {
        store: Mutex<HashMap<String, Value>>,
    }

    #[async_trait]
    impl Cache for MockCache {
        async fn set(
            &self,
            key: &str,
            value: &Value,
            _timeout_s: Option<u32>,
        ) -> Result<(), CacheError> {
            self.store
                .lock()
                .await
                .insert(key.to_string(), value.clone());
            Ok(())
        }

        async fn get(&self, key: &str) -> Result<Option<Value>, CacheError> {
            Ok(self.store.lock().await.get(key).cloned())
        }

        async fn delete(&self, key: &str) -> Result<(), CacheError> {
            self.store.lock().await.remove(key);
            Ok(())
        }

        async fn set_many(
            &self,
            mappings: &HashMap<String, Value>,
            _timeout_s: Option<u32>,
        ) -> Result<(), CacheError> {
            let mut store = self.store.lock().await;
            for (key, value) in mappings {
                store.insert(key.clone(), value.clone());
            }
            Ok(())
        }

        async fn get_many(
            &self,
            keys: &[&str],
        ) -> Result<HashMap<String, Value>, CacheError> {
            let store = self.store.lock().await;
            Ok(keys
                .iter()
                .filter_map(|key| store.get(*key).map(|v| (key.to_string(), v.clone())))
                .collect())
        }

        async fn delete_many(&self, keys: &[&str]) -> Result<(), CacheError> {
            let mut store = self.store.lock().await;
            for key in keys {
                store.remove(*key);
            }
            Ok(())
        }
    }

    fn make_keycloak(
        cache: Arc<RwLock<dyn Cache>>,
        database: Arc<RwLock<dyn Database>>,
    ) -> Keycloak {
        Keycloak {
            provider_url: String::new(),
            audiences: Vec::new(),
            keys: None,
            cache,
            database,
        }
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
    fn realm_from_iss_extracts_last_segment() {
        let iss = "http://localhost:8090/realms/master";
        assert_eq!(
            realm_from_iss(iss),
            Some("master".to_string()),
            "realm should be the last path segment of iss={iss}"
        );
    }

    #[test]
    fn realm_from_iss_ignores_trailing_slash() {
        let iss = "http://localhost:8090/realms/my-realm/";
        assert_eq!(
            realm_from_iss(iss),
            Some("my-realm".to_string()),
            "trailing slash should be trimmed for iss={iss}"
        );
    }

    #[test]
    fn realm_from_iss_without_slash_returns_input() {
        let iss = "standalone";
        assert_eq!(
            realm_from_iss(iss),
            Some("standalone".to_string()),
            "an iss without '/' should yield itself, iss={iss}"
        );
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
        let cache: Arc<RwLock<dyn Cache>> = Arc::new(RwLock::new(MockCache::default()));
        let database: Arc<RwLock<dyn Database>> =
            Arc::new(RwLock::new(MockDatabase::default()));
        let keycloak = make_keycloak(cache, database);

        let hashed = hex_sha256("my-secret-token");
        let token = UserToken {
            id: Uuid::new_v4(),
            realm: "api_key".to_string(),
        };

        assert!(
            keycloak.try_get_cache(&hashed).await.is_none(),
            "cache should be empty before set, hashed={hashed}"
        );

        keycloak.set_cache(&token, &hashed).await;

        let cached = keycloak
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
        let cache: Arc<RwLock<dyn Cache>> = Arc::new(RwLock::new(MockCache::default()));
        let database: Arc<RwLock<dyn Database>> =
            Arc::new(RwLock::new(MockDatabase::default()));
        let keycloak = make_keycloak(cache, database);

        assert!(
            keycloak.try_get_cache("unknown-hash").await.is_none(),
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

        let cache: Arc<RwLock<dyn Cache>> = Arc::new(RwLock::new(MockCache::default()));
        let database: Arc<RwLock<dyn Database>> = Arc::new(RwLock::new(MockDatabase {
            api_keys_by_hash,
            ..Default::default()
        }));
        let keycloak = make_keycloak(cache, database);

        let user_token = keycloak
            .validate_api_key(token)
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

        let cached = keycloak
            .try_get_cache(&hashed)
            .await
            .expect("validate_api_key should populate the cache");
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

        let cache: Arc<RwLock<dyn Cache>> = Arc::new(RwLock::new(MockCache::default()));
        // Empty database: a cache miss would surface as AuthenticationFailure.
        let database: Arc<RwLock<dyn Database>> =
            Arc::new(RwLock::new(MockDatabase::default()));
        let keycloak = make_keycloak(cache, database);

        keycloak
            .set_cache(
                &UserToken {
                    id: owner,
                    realm: "api_key".to_string(),
                },
                &hashed,
            )
            .await;

        let user_token = keycloak
            .validate_api_key(token)
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
        let cache: Arc<RwLock<dyn Cache>> = Arc::new(RwLock::new(MockCache::default()));
        let database: Arc<RwLock<dyn Database>> =
            Arc::new(RwLock::new(MockDatabase::default()));
        let keycloak = make_keycloak(cache, database);

        let result = keycloak.validate_api_key("does-not-exist").await;
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
