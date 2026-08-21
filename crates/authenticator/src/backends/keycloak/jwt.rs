//! RS256 validation of provider-issued access tokens against the realm's JWKS.

use jsonwebtoken::jwk::JwkSet;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use serde::Deserialize;
use tokio::sync::RwLock;
use tracing::warn;
use uuid::Uuid;

use crate::error::{AuthenticatorError, error_chain};
use crate::models::UserToken;

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

/// RS256 validation of provider-issued access tokens against the realm's JWKS.
pub(super) struct JwtValidator {
    jwks_url: String,
    audiences: Vec<String>,
    keys: RwLock<Option<JwkSet>>,
}

impl JwtValidator {
    /// Fetches the realm's signing keys, tolerating a provider that is not up
    /// yet: the failure is only warned about, and the next [`Self::validate`]
    /// retries the fetch. Once the keys are in, they are reused as-is.
    pub(super) async fn new(jwks_url: &str, audiences: Vec<String>) -> Self {
        let validator = Self {
            jwks_url: jwks_url.to_string(),
            audiences,
            keys: RwLock::new(None),
        };
        if let Err(e) = validator.refresh().await {
            warn!(
                "Could not reach the authentication server yet: {}",
                error_chain(e.as_ref())
            );
        }
        validator
    }

    async fn refresh(&self) -> Result<(), Box<AuthenticatorError>> {
        let keys: JwkSet = reqwest::get(&self.jwks_url).await?.json().await?;
        if keys.keys.is_empty() {
            return Err(Box::new(AuthenticatorError::NoJwk));
        }
        *self.keys.write().await = Some(keys);
        Ok(())
    }

    /// Returns the key matching `kid`, fetching the JWKS first if the provider
    /// was still unreachable when the validator was built.
    async fn decoding_key(
        &self,
        kid: &str,
    ) -> Result<DecodingKey, Box<AuthenticatorError>> {
        if self.keys.read().await.is_none() {
            self.refresh().await?;
        }
        let keys = self.keys.read().await;
        let jwk = keys
            .as_ref()
            .ok_or_else(|| Box::new(AuthenticatorError::NoJwk))?
            .find(kid)
            .ok_or("No matching key found in JWKS")?;
        Ok(DecodingKey::from_jwk(jwk)?)
    }

    pub(super) async fn validate(
        &self,
        token: &str,
    ) -> Result<UserToken, Box<AuthenticatorError>> {
        let header = decode_header(token)?;
        let kid = header.kid.ok_or("No 'kid' in token header")?;
        let decoding_key = self.decoding_key(&kid).await?;

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&self.audiences);

        let claims = decode::<Claims>(token, &decoding_key, &validation)?.claims;
        let id = Uuid::parse_str(&claims.sub)
            .map_err(|e| AuthenticatorError::Message(format!("invalid sub UUID: {e}")))?;
        let realm = realm_from_iss(&claims.iss)
            .ok_or(AuthenticatorError::InvalidRealm(claims.iss))?;

        Ok(UserToken { id, realm })
    }
}

test_utils::tests_file!("_tests/test_jwt.rs");
