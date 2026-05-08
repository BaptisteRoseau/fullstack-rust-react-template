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
    #[allow(dead_code)]
    sub: String,
    #[allow(dead_code)]
    iss: String,
    #[allow(dead_code)]
    exp: usize,
    id: Uuid,
    realm: String,
}

impl From<Claims> for UserToken {
    fn from(claims: Claims) -> Self {
        UserToken {
            id: claims.id,
            realm: claims.realm,
        }
    }
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

        let decoded_token = decode::<Claims>(token, &decoding_key, &validation)?;
        Ok(decoded_token.claims.into())
    }

    async fn validate_api_key(&self, token: &str) -> Result<UserToken, Box<AuthenticatorError>> {
        let hashed = hex_sha256(token);

        if let Some(value) = self.cache.read().await.get_nofail(&hashed).await
            && let Ok(user_token) = serde_json::from_value::<UserToken>(value)
        {
            return Ok(user_token);
        }

        // todo!() - look up the hashed API key in the database
        // if let Some(user_token) = self.database.read().await.get_api_key(&hashed).await? {
        //     return Ok(user_token);
        // }

        Err(Box::new(AuthenticatorError::AuthenticationFailure))
    }
}

fn hex_sha256(input: &str) -> String {
    Sha256::digest(input.as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
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
