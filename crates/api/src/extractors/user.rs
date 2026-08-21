use crate::endpoints::auth::cookies::ACCESS_COOKIE;
use crate::models::UserToken;
use crate::{AppState, error::ApiError, extractors::error::ExtractorError};
use authenticator::error::AuthenticatorError;
use axum::{
    RequestPartsExt,
    extract::{FromRef, FromRequestParts, OptionalFromRequestParts},
    http::{HeaderMap, header, request::Parts},
};
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::errors::ErrorKind as JwtErrorKind;
use tracing::{Span, debug};

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

/// Whether a validation failure must keep the status [`AuthenticatorError`]
/// already maps it to, instead of being reduced to "invalid credential".
///
/// Two cases qualify. A genuine server-side fault — the provider is unreachable,
/// or its signing keys were never fetched — is ours, not the caller's, and must
/// stay a 500. An expired token has its own 401 flavour that tells the frontend
/// to silently refresh rather than to log the user out.
fn keeps_own_status(error: &AuthenticatorError) -> bool {
    match error {
        AuthenticatorError::NoJwk | AuthenticatorError::RequestError(_) => true,
        AuthenticatorError::JwtError(jwt_error) => {
            matches!(jwt_error.kind(), JwtErrorKind::ExpiredSignature)
        }
        _ => false,
    }
}

/// Maps a failure to validate a caller-supplied token onto an API error.
///
/// Everything the authenticator rejects here describes a credential the caller
/// sent: an unknown signing key (the provider restarted or rotated its keys
/// while the browser kept its cookie), a malformed header, an unparseable
/// subject, a foreign realm... None of those are server faults, so they must
/// answer 401 and let the caller re-authenticate instead of surfacing a 500.
fn credential_error(error: Box<AuthenticatorError>) -> ApiError {
    if keeps_own_status(&error) {
        return ApiError::AuthenticatorError(error);
    }
    debug!("Rejecting an unusable credential: {error}");
    ApiError::ExtractorError(ExtractorError::InvalidJwt)
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
            user = authenticator
                .validate(&token)
                .await
                .map_err(credential_error)?;
        }
        let user: UserToken = user.into();
        Span::current().record("user_id", tracing::field::display(user.id));
        Ok(Some(user))
    }
}

test_utils::tests_file!("_tests/test_user.rs");
