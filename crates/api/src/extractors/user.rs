use crate::models::UserToken;
use crate::{AppState, error::ApiError, extractors::error::ExtractorError};
use axum::{
    RequestPartsExt,
    extract::{FromRef, FromRequestParts, OptionalFromRequestParts},
    http::{HeaderMap, header, request::Parts},
};
use tracing::debug;

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
        match <Option<Self>>::from_request_parts(parts, state).await {
            Ok(opt_user) => match opt_user {
                Some(user) => Ok(user),
                None => Err(ApiError::from(ExtractorError::NotLoggedIn)),
            },
            Err(e) => Err(e),
        }
    }
}

impl<S> OptionalFromRequestParts<S> for UserToken
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
    ) -> Result<Option<Self>, Self::Rejection> {
        let headers = HeaderMap::from_request_parts(parts, state)
            .await
            .map_err(anyhow::Error::from)?;
        let header = match headers.get(header::AUTHORIZATION) {
            Some(header) => header,
            None => {
                debug!("Anonymous user");
                return Ok(None);
            }
        };

        let raw = header.to_str().map_err(ExtractorError::from)?;
        let token = raw.strip_prefix("Bearer ").unwrap_or(raw);

        let app_state = parts.extract_with_state::<AppState, _>(state).await?;
        let user;
        {
            let authenticator = app_state.authenticator.read().await;
            user = authenticator.validate(token).await?;
        }
        Ok(Some(user.into()))
    }
}
