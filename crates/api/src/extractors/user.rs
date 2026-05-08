use crate::models::UserToken;
use crate::{AppState, error::ApiError, extractors::errors::ExtractorError};
use axum::{
    RequestPartsExt,
    extract::{FromRef, FromRequestParts},
    http::{HeaderMap, header, request::Parts},
};
use tracing::debug;

pub struct OptionalUser(Option<UserToken>);

impl OptionalUser {
    pub fn inner(self) -> Option<UserToken> {
        self.0
    }
}

impl From<OptionalUser> for Option<UserToken> {
    fn from(value: OptionalUser) -> Self {
        value.0
    }
}

impl<S> FromRequestParts<S> for OptionalUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
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

        let raw = header.to_str().map_err(|e| anyhow::anyhow!(e))?;
        let token = raw.strip_prefix("Bearer ").unwrap_or(raw);

        let app_state = parts.extract_with_state::<AppState, _>(state).await?;
        let user;
        {
            let authenticator = app_state.authenticator.read().await;
            user = authenticator.validate(token).await?;
        }
        Ok(OptionalUser(Some(user.into())))
    }
}

impl<S> FromRequestParts<S> for UserToken
where
    S: Send + Sync,
    AppState: FromRef<S>,
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
