use crate::{Authenticator, UserInfo, error::AuthenticatorError};
use async_trait::async_trait;
use cache::Cache;
use config::Config;
use database::Database;
use jsonwebtoken::jwk::JwkSet;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use serde::Deserialize;
use std::collections::{BTreeMap, HashSet};
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
    iss: String, // Issuer (should match your Authentik URL)
    exp: usize,  // Expiration time
    id: Uuid,
    groups: Option<HashSet<Uuid>>,
    roles: Option<HashSet<Uuid>>,
}

impl Into<UserInfo> for Claims {
    fn into(self) -> UserInfo {
        UserInfo {
            id: self.id,
            groups: self.groups.unwrap_or_default(),
            roles: self.roles.unwrap_or_default(),
        }
    }
}

pub struct SecretsProvider {
    provider_url: String,
    audiences: Vec<String>,
    keys: Option<JwkSet>,
    token2userinfo: BTreeMap<String, UserInfo>,
    cache: Arc<RwLock<dyn Cache>>,
    database: Arc<RwLock<dyn Database>>,
}

impl From<&Config> for SecretsProvider {
    fn from(value: &Config) -> Self {
        todo!()
    }
}

impl SecretsProvider {
    async fn refresh_keys(&mut self) -> Result<UserInfo, Box<AuthenticatorError>> {
        todo!()
        // self.keys = Some(reqwest::get(self.provider_url).await?.json().await?);
    }

    async fn validate_jwt(
        &self,
        token: &str,
    ) -> Result<UserInfo, Box<AuthenticatorError>> {
        todo!()
        // let header = decode_header(token)?;
        // let kid = header.kid.ok_or("No 'kid' in token header")?;
        // let jwk = self
        //     .keys
        //     .find(&kid)
        //     .ok_or("No matching key found in JWKS")?;
        // let decoding_key = DecodingKey::from_jwk(jwk)?;

        // let mut validation = Validation::new(Algorithm::RS256);
        // validation.set_audience(&self.audience);

        // let decoded_token = decode::<Claims>(token, &decoding_key, &validation)?;
        // Ok(decoded_token.claims.into())
    }

    async fn validate_api_key(
        &self,
        token: &str,
    ) -> Result<UserInfo, Box<AuthenticatorError>> {
        // TODO: Fetch API key in the cache, fallback to DB
        todo!()
    }
}

#[async_trait]
impl Authenticator for SecretsProvider {
    async fn validate(&self, token: &str) -> Result<UserInfo, Box<AuthenticatorError>> {
        if let Some(info) = self.token2userinfo.get(token) {
            return Ok(info.clone());
        }
        match self.validate_jwt(token).await {
            Ok(info) => Ok(info),
            Err(e) => match e {
                //TODO: valid error cases requiring fallback to API key
                // => return self.validate_api_key(token).await,
                _ => Err(Box::new(AuthenticatorError::AuthenticationFailure)),
            },
        }
    }
}
