//! RS256 validation of provider-issued access tokens against the realm's JWKS.

use jsonwebtoken::jwk::JwkSet;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use serde::Deserialize;
use uuid::Uuid;

use crate::error::AuthenticatorError;
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
    keys: JwkSet,
    audiences: Vec<String>,
}

impl JwtValidator {
    /// Fetches the realm's signing keys once, at construction. Every call to
    /// [`Self::validate`] afterwards reuses these keys — the JWKS is never
    /// re-fetched.
    pub(super) async fn fetch(
        jwks_url: &str,
        audiences: Vec<String>,
    ) -> Result<Self, Box<AuthenticatorError>> {
        let keys: JwkSet = reqwest::get(jwks_url).await?.json().await?;
        if keys.keys.is_empty() {
            return Err(Box::new(AuthenticatorError::NoJwk));
        }
        Ok(Self { keys, audiences })
    }

    pub(super) fn validate(
        &self,
        token: &str,
    ) -> Result<UserToken, Box<AuthenticatorError>> {
        let header = decode_header(token)?;
        let kid = header.kid.ok_or("No 'kid' in token header")?;
        let jwk = self
            .keys
            .find(&kid)
            .ok_or("No matching key found in JWKS")?;
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
}

#[cfg(test)]
#[path = "_tests/test_jwt.rs"]
mod tests;
