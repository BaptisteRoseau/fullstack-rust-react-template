use crate::models::UserToken;
use crate::{AppState, error::ApiError, extractors::error::ExtractorError};
use axum::{
    RequestPartsExt,
    extract::{FromRef, FromRequestParts, OptionalFromRequestParts},
    http::{HeaderMap, header, request::Parts},
};
use axum_extra::extract::cookie::CookieJar;
use tracing::debug;

/// Name of the httpOnly cookie holding the access token (set by the auth BFF).
const ACCESS_COOKIE: &str = "access_token";

/// Extracts the bearer/API-key token from the `Authorization` header, falling back
/// to the `access_token` cookie set by the OAuth Backend-for-Frontend.
fn extract_token(headers: &HeaderMap) -> Result<Option<String>, ExtractorError> {
    if let Some(header) = headers.get(header::AUTHORIZATION) {
        let raw = header.to_str().map_err(ExtractorError::from)?;
        return Ok(Some(raw.strip_prefix("Bearer ").unwrap_or(raw).to_string()));
    }

    let token = CookieJar::from_headers(headers)
        .get(ACCESS_COOKIE)
        .map(|cookie| cookie.value().to_string());
    Ok(token)
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
        let token = match extract_token(&headers)? {
            Some(token) => token,
            None => {
                debug!("Anonymous user");
                return Ok(None);
            }
        };

        let app_state = parts.extract_with_state::<AppState, _>(state).await?;
        let user;
        {
            let authenticator = app_state.authenticator.read().await;
            user = authenticator.validate(&token).await?;
        }
        Ok(Some(user.into()))
    }
}
