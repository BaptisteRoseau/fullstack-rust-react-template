use crate::{Authenticator, UserInfo, error::AuthenticatorError};
use async_trait::async_trait;
use config::Config;
use jsonwebtoken::jwk::JwkSet;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use serde::Deserialize;
use std::collections::{BTreeMap, HashSet};
use uuid::Uuid;

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
    audience: Vec<String>,
    keys: Option<JwkSet>,
    token2userinfo: BTreeMap<String, UserInfo>,
    // Should also have a Database and cache to check for API keys
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
