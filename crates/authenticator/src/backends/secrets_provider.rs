use crate::{Authenticator, UserToken, error::AuthenticatorError};
use async_trait::async_trait;
use cache::Cache;
use config::Config;
use database::Database;
use jsonwebtoken::jwk::JwkSet;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

// Claude prompt for the next steps:
// Look at the @crates/authenticator/ crate. It implements a JWT decoding logic from a service that exposes JWK keys. I want you to: 1.
// Add the audiences and provider_url in the config, defaulting to keycloak default endpoint (see
// @infrastructure/docker-compose/docker-compose.authentication.yml  ), 2. uncomment the
// @crates/authenticator/src/backends/secrets_provider.rs code, 3. fix the conversion errors from Box<> in
// @crates/authenticator/src/error.rs, 3. Similarly to the ApiState, add Arc<Rwlock<dyn Database>> and Arc<Rwlock<dyn Cache>> and use them
// to retrieve the API keys in the corresponding function, consider the function for the DB simply exist and comment it out and add a
// todo!() instruction above, we will implement it later. That's it for now, wait for my input next. use cargo clippy to validate your
// work, rust-analyzer LSP to navigate in the code, commit for each step, and ignore "unused" errors.

#[derive(Debug, Deserialize)]
struct Claims {
    sub: String, // User's unique ID
    iss: String, // Issuer (should match your Keyckoak URL)
    exp: usize,  // Expiration time
    id: Uuid,
    realm: String,
}

impl Into<UserToken> for Claims {
    fn into(self) -> UserToken {
        UserToken {
            id: self.id,
            realm: self.realm,
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
    fn try_new(
        config: &Config,
        cache: Arc<RwLock<dyn Cache>>,
        database: Arc<RwLock<dyn Database>>,
    ) -> Result<Self, Box<AuthenticatorError>> {
        todo!("Add config for authenticator");
        // let mut authenticator = Self {
        //     provider_url: config.authenticator.provider_url,
        //     audiences: config.authenticator.audiences,
        //     keys: None,
        //     cache,
        //     database,
        // };
        // authenticator.refresh()?;
        // authenticator
    }

    async fn validate_jwt(
        &self,
        token: &str,
    ) -> Result<UserToken, Box<AuthenticatorError>> {
        let header = decode_header(token)?;
        let kid = header.kid.ok_or("No 'kid' in token header")?;
        let jwk = self
            .keys
            .map(|f| f.find(&kid).ok_or("No matching key found in JWKS"))?;
        // Err(Box::new(AuthenticatorError::NoJwk)
        let decoding_key = DecodingKey::from_jwk(jwk)?;

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&self.audience);

        let decoded_token = decode::<Claims>(token, &decoding_key, &validation)?;
        Ok(decoded_token.claims.into())
    }

    async fn validate_api_key(
        &self,
        token: &str,
    ) -> Result<UserToken, Box<AuthenticatorError>> {
        // TODO:
        // 1. Hash API key (sha256 ? bcrypt ?)
        // 2. Fetch API in cache -> OK return
        // 3. Fallback to DB -> OK return
        // 4. Return error
        // let hashed = "Should hash the token";
        // if let Some(_match) = self.cache.read().await.get_nofail(&hashed).await {
        //     let user_token = 
        //     return Ok(user_token);
        // }

        // if let Some(_match) = self.database.read().await. {
            
        // }

        Err(Box::new(AuthenticatorError::AuthenticationFailure))
    }
}

#[async_trait]
impl Authenticator for SecretsProvider {
    async fn validate(&self, token: &str) -> Result<UserToken, Box<AuthenticatorError>> {
        // Only JWT contain dots
        match token.contains('.') {
            true => match self.validate_jwt(token).await {
                Ok(info) => Ok(info),
                Err(e) => Err(e),
            },
            false => match self.validate_api_key(token).await {
                Ok(info) => Ok(info),
                Err(e) => Err(e),
            },
        }
    }

    async fn refresh(&mut self) -> Result<(), Box<AuthenticatorError>> {
        self.keys = Some(reqwest::get(&self.provider_url).await?.json().await?);
        Ok(())
    }
}
