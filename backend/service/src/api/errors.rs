use crate::databases::errors::DatabaseError;
use axum::{http::StatusCode, response::IntoResponse};
use jsonwebtoken::errors::ErrorKind as JwtErrorKind;
use serde::Serialize;
use thiserror::Error;
use tracing::error;
use utoipa::ToSchema;

/// This is the API standard struct supposed to be sent
/// as a response every time an error occurs.
///
/// Should only be used through `ApiError` enum.
#[derive(Serialize, ToSchema)]
struct ApiErrorResponse {
    id: String,
    error: String,
    #[serde(skip_serializing)]
    status_code: StatusCode,
}

impl ApiErrorResponse {
    fn new<I, E>(id: I, error: E, code: StatusCode) -> Self
    where
        I: ToString,
        E: ToString,
    {
        Self {
            id: id.to_string(),
            error: error.to_string(),
            status_code: code,
        }
    }

    /// Template for unexpected error
    fn unexpected() -> Self {
        Self::new(
            "UNEXPECTED",
            "An unexpected error occurred.",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    }

    // Template for forbidden responses
    fn forbidden() -> Self {
        Self::new("FORBIDDEN", "", StatusCode::FORBIDDEN)
    }

    // Template for not found responses
    fn not_found() -> Self {
        Self::new("NOT_FOUND", "", StatusCode::NOT_FOUND)
    }
}

impl IntoResponse for ApiErrorResponse {
    fn into_response(self) -> axum::response::Response {
        // Do not return a body in case of a forbidden or not found
        // error code.
        match self.status_code {
            StatusCode::FORBIDDEN | StatusCode::NOT_FOUND => self.status_code.into_response(),
            _ => (self.status_code, axum::response::Json(self)).into_response(),
        }
    }
}

/// API list of all errors that can happen in the backend.
///
/// The errors can be made into an API response using the
/// `ApiErrorResponse` structure to automatically send them back in the
/// HTTP API though Axum's error management.
///
/// The error message will be logged but not sent in the server response.
#[derive(Error, Debug)]
pub(crate) enum ApiError {
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Hardware Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    Database(#[from] DatabaseError),
    // #[error("Serialization error")]
    // Serde(#[from] serde::err),
    #[error("Unexpected Error")]
    Unexpected(#[from] anyhow::Error),
}

impl From<ApiError> for ApiErrorResponse {
    fn from(val: ApiError) -> Self {
        match val {
            ApiError::NotFound(_) => ApiErrorResponse::not_found(),
            ApiError::IoError(_) => ApiErrorResponse::unexpected(),
            ApiError::Database(e) => e.into(),
            ApiError::Unexpected(e) => e.into(),
        }
    }
}

impl From<DatabaseError> for ApiErrorResponse {
    fn from(val: DatabaseError) -> Self {
        match val {
            DatabaseError::NotFound(_) => ApiErrorResponse::not_found(),
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
                "TOKEN_EXPIRED",
                "Your authentication token has expired. Please log back in.",
                StatusCode::UNAUTHORIZED,
            ),
            _ => ApiErrorResponse::unexpected(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        error!("API ERROR: {:?}", &self);
        let api_error: ApiErrorResponse = self.into();
        api_error.into_response()
    }
}
