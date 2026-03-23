use axum::{
    extract::FromRequestParts,
    http::{HeaderMap, header, request::Parts},
};
use tracing::debug;
use uuid::Uuid;

use crate::{error::ApiError, extractors::errors::ExtractorError, models::User};

pub struct OptionalUser(Option<User>);

impl OptionalUser {
    pub fn inner(self) -> Option<User> {
        self.0
    }
}

impl From<OptionalUser> for Option<User> {
    fn from(value: OptionalUser) -> Self {
        value.0
    }
}

impl<S> FromRequestParts<S> for OptionalUser
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let headers = HeaderMap::from_request_parts(parts, state)
            .await
            .map_err(anyhow::Error::from)?;
        let header = match headers.get(header::AUTHORIZATION) {
            Some(header) => header,
            None => {
                debug!("Anonymous user");
                return Ok(OptionalUser(None));
            }
        };

        let token: String = header.to_str().unwrap().to_string();
        let _token = token
            .strip_prefix("Bearer ")
            .unwrap_or(token.as_str())
            .to_string();
        let user = User::new(&Uuid::now_v7(), &"name");
        Ok(OptionalUser(Some(user)))
    }
}

impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    /// Extract the user if possible. If not, directly return an error without getting to
    /// the handler.
    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        match OptionalUser::from_request_parts(parts, state).await {
            Ok(opt_user) => match opt_user.inner() {
                Some(user) => Ok(user),
                None => Err(ApiError::from(ExtractorError::NotLoggedIn)),
            },
            Err(e) => Err(e),
        }
    }
}
