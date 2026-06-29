use crate::{error::ApiError, extractors::error::ExtractorError};
use app_core::error::CoreError;
use authenticator::error::AuthenticatorError;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use jsonwebtoken::errors::ErrorKind as JwtErrorKind;
use serde::Serialize;
use tower_governor::errors::GovernorError;
use utoipa::ToSchema;

/// An enum representing and API error.
// TODO: Serde uppercanse
#[derive(Serialize, ToSchema)]
pub(crate) enum ApiErrorId {
    Unexpected,
    Unauthorized,
    Forbidden,
    TokenExpired,
    NotFound,
    TooManyRequests,
    HeaderInvalidAsciiCharacters,
}

/// This is the standard API error returned by endpoints.
#[derive(Serialize, ToSchema)]
pub(crate) struct ApiErrorResponse {
    pub id: ApiErrorId,
    pub error: String,
    #[serde(skip_serializing)]
    pub status_code: StatusCode,
}

impl ApiErrorResponse {
    fn new<I, E>(id: I, error: E, code: StatusCode) -> Self
    where
        I: Into<ApiErrorId>,
        E: ToString,
    {
        Self {
            id: id.into(),
            error: error.to_string(),
            status_code: code,
        }
    }

    /// Template for unexpected error
    fn unexpected() -> Self {
        Self::new(
            ApiErrorId::Unexpected,
            "An unexpected error occurred.",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    }

    /// Template for forbidden responses
    fn unauthorized() -> Self {
        Self::new(
            ApiErrorId::Unauthorized,
            "You need to be logged in to access this ressource",
            StatusCode::UNAUTHORIZED,
        )
    }

    /// Template for forbidden responses
    fn forbidden() -> Self {
        Self::new(
            ApiErrorId::Forbidden,
            "You are not allowed to access this ressource.",
            StatusCode::FORBIDDEN,
        )
    }

    /// Template for not found responses
    fn not_found() -> Self {
        Self::new(ApiErrorId::NotFound, "Not found.", StatusCode::NOT_FOUND)
    }

    /// Template for rate-limited responses
    fn too_many_requests() -> Self {
        Self::new(
            ApiErrorId::TooManyRequests,
            "Too many requests. Please slow down and try again later.",
            StatusCode::TOO_MANY_REQUESTS,
        )
    }
}

impl IntoResponse for ApiErrorResponse {
    fn into_response(self) -> axum::response::Response {
        (self.status_code, axum::response::Json(self)).into_response()
    }
}

/* =======================================================================================
 * CONVERSIONS

This block contains conversion for errors into ApiErrorResponse.
Every error from the whole backend should be convertible into ApiErrorResponse,
which is the single type of response returned by the API.

Errors should be converted using a `match` arm as follows:

```
 impl From<MyCustomError> for ApiErrorResponse {
 fn from(_val: MyCustomError) -> Self {
         MyCustomError::Unexpected => { ApiErrorResponse::unexpected() },
         MyCustomError::TokenExpired =>  ApiErrorResponse::new(
             ApiErrorId::TokenExpired,
             "Your authentication token has expired. Please log back in.",
             StatusCode::UNAUTHORIZED,
         )
     }
 }
 ```
======================================================================================= */

impl From<ApiError> for ApiErrorResponse {
    fn from(val: ApiError) -> Self {
        match val {
            ApiError::NotFound(_) => ApiErrorResponse::not_found(),
            ApiError::IoError(_) => ApiErrorResponse::unexpected(),
            ApiError::CoreError(e) => e.into(),
            ApiError::ExtractorError(e) => e.into(),
            ApiError::StorageError(_) => ApiErrorResponse::unexpected(),
            ApiError::TooManyRequests => ApiErrorResponse::too_many_requests(),
            ApiError::Unexpected(e) => e.into(),
            ApiError::AuthenticatorError(e) => e.into(),
        }
    }
}

impl From<ExtractorError> for ApiErrorResponse {
    fn from(val: ExtractorError) -> Self {
        match val {
            ExtractorError::InvalidJwt | ExtractorError::NotLoggedIn => {
                ApiErrorResponse::unauthorized()
            }
            ExtractorError::Unexpected(_) => ApiErrorResponse::unexpected(),
            ExtractorError::InvalidHeaderCharacters(_) => ApiErrorResponse::new(
                ApiErrorId::HeaderInvalidAsciiCharacters,
                "A Header contains invalid Ascii characters. Remove them to continue.",
                StatusCode::BAD_REQUEST,
            ),
        }
    }
}

impl From<CoreError> for ApiErrorResponse {
    fn from(val: CoreError) -> Self {
        match val {
            CoreError::NotFound(_) => ApiErrorResponse::not_found(),
            _ => ApiErrorResponse::unexpected(),
        }
    }
}

impl From<anyhow::Error> for ApiErrorResponse {
    fn from(_val: anyhow::Error) -> Self {
        ApiErrorResponse::unexpected()
    }
}

impl From<jsonwebtoken::errors::Error> for ApiErrorResponse {
    fn from(val: jsonwebtoken::errors::Error) -> Self {
        match val.kind() {
            JwtErrorKind::InvalidToken
            | JwtErrorKind::InvalidSubject
            | JwtErrorKind::InvalidIssuer
            | JwtErrorKind::InvalidSignature => ApiErrorResponse::forbidden(),
            JwtErrorKind::ExpiredSignature => ApiErrorResponse::new(
                ApiErrorId::TokenExpired,
                "Your authentication token has expired. Please log back in.",
                StatusCode::UNAUTHORIZED,
            ),
            _ => ApiErrorResponse::unexpected(),
        }
    }
}

impl From<Box<AuthenticatorError>> for ApiErrorResponse {
    fn from(val: Box<AuthenticatorError>) -> Self {
        match *val {
            AuthenticatorError::InvalidSignature
            | AuthenticatorError::AuthenticationFailure => Self::forbidden(),
            _ => ApiErrorResponse::unexpected(),
        }
    }
}

impl From<GovernorError> for ApiError {
    fn from(val: GovernorError) -> Self {
        match val {
            GovernorError::TooManyRequests { .. } => ApiError::TooManyRequests,
            other => ApiError::Unexpected(other.into()),
        }
    }
}
