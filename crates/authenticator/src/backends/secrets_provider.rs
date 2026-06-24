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

#[derive(Debug, Deserialize, Default)]
struct RealmAccess {
    #[serde(default)]
    roles: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Claims {
    sub: String,
    iss: String,
    #[serde(default)]
    email: Option<String>,
    #[serde(default)]
    realm_access: RealmAccess,
}

// Keycloak sets iss to "http://<host>/realms/<realm-name>"; extract the last segment.
fn realm_from_iss(iss: &str) -> String {
    iss.trim_end_matches('/')
        .rsplit('/')
        .next()
        .unwrap_or("")
        .to_string()
}

pub struct SecretsProvider {
    provider_url: String,
    audiences: Vec<String>,
    keys: Option<JwkSet>,
    cache: Arc<RwLock<dyn Cache>>,
    database: Arc<RwLock<dyn Database>>,
}

impl SecretsProvider {
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

    async fn validate_jwt(&self, token: &str) -> Result<UserToken, Box<AuthenticatorError>> {
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
        Ok(UserToken {
            id,
            realm: realm_from_iss(&claims.iss),
            email: claims.email,
            roles: claims.realm_access.roles,
        })
    }

    async fn validate_api_key(&self, token: &str) -> Result<UserToken, Box<AuthenticatorError>> {
        let hashed = hex_sha256(token);

        if let Some(value) = self.cache.read().await.get_nofail(&hashed).await
            && let Ok(user_token) = serde_json::from_value::<UserToken>(value)
        {
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
            email: None,
            roles: vec![],
        };

        if let Ok(value) = serde_json::to_value(&user_token) {
            self.cache
                .read()
                .await
                .set_nofail(&hashed, &value, Some(300))
                .await;
        }

        Ok(user_token)
    }
}

fn hex_sha256(input: &str) -> String {
    Sha256::digest(input.as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claims_deserialize_email_and_roles() {
        let json = r#"{
            "sub": "550e8400-e29b-41d4-a716-446655440000",
            "iss": "http://localhost:8090/realms/application",
            "email": "user@example.com",
            "realm_access": {
                "roles": ["ADMIN", "USER"]
            }
        }"#;

        let claims: Claims =
            serde_json::from_str(json).expect("claims JSON should deserialize without error");

        assert_eq!(
            claims.sub, "550e8400-e29b-41d4-a716-446655440000",
            "sub mismatch: got {:?}",
            claims.sub
        );
        assert_eq!(
            claims.email,
            Some("user@example.com".to_string()),
            "email mismatch: got {:?}",
            claims.email
        );
        assert_eq!(
            claims.realm_access.roles,
            vec!["ADMIN", "USER"],
            "roles mismatch: got {:?}",
            claims.realm_access.roles
        );
    }

    #[test]
    fn test_claims_deserialize_missing_optional_fields() {
        let json = r#"{
            "sub": "550e8400-e29b-41d4-a716-446655440000",
            "iss": "http://localhost:8090/realms/application"
        }"#;

        let claims: Claims =
            serde_json::from_str(json).expect("claims JSON with no optional fields should deserialize");

        assert!(
            claims.email.is_none(),
            "email should be None when absent, got {:?}",
            claims.email
        );
        assert!(
            claims.realm_access.roles.is_empty(),
            "roles should be empty when absent, got {:?}",
            claims.realm_access.roles
        );
    }

    #[test]
    fn test_realm_from_iss() {
        let realm = realm_from_iss("http://localhost:8090/realms/application");
        assert_eq!(
            realm, "application",
            "realm extraction mismatch: got {:?}",
            realm
        );
    }
}

#[async_trait]
impl Authenticator for SecretsProvider {
    async fn validate(&self, token: &str) -> Result<UserToken, Box<AuthenticatorError>> {
        // Only JWT contain dots
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
