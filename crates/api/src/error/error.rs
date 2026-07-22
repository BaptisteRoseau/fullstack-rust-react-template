use app_core::error::CoreError;
use authenticator::error::AuthenticatorError;
use axum::{http::StatusCode, response::IntoResponse};
use std::fmt::Debug;
use tower_governor::GovernorError;

use storage::error::StorageError;

use crate::{error::ApiErrorResponse, extractors::error::ExtractorError};

/// API list of all errors that can happen in the backend.
///
/// The errors can be made into an API response using the
/// `ApiErrorResponse` structure to automatically send them back in the
/// HTTP API though Axum's error management.
///
/// The error message will be logged but not sent in the server response.
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Hardware Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    CoreError(#[from] CoreError),
    #[error(transparent)]
    ExtractorError(#[from] ExtractorError),
    #[error(transparent)]
    AuthenticatorError(#[from] Box<AuthenticatorError>),
    #[error("Storage Error: {0}")]
    StorageError(#[from] Box<StorageError>),
    #[error("Too many requests, wait for {0} seconds before retrying")]
    TooManyRequests(u64),
    #[error("Unexpected Error")]
    Unexpected(#[from] anyhow::Error),
}

impl From<GovernorError> for ApiError {
    fn from(val: GovernorError) -> Self {
        match val {
            GovernorError::TooManyRequests {
                wait_time,
                headers: _,
            } => ApiError::TooManyRequests(wait_time),
            other => ApiError::Unexpected(other.into()),
        }
    }
}

/* =======================================================================================
* API RESPONSE CONVERSION & ERROR LOGGING
======================================================================================= */

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let error_trace = format!("{:?}", self);
        let api_error: ApiErrorResponse = self.into();
        log_error_trace(api_error.status_code, error_trace);
        api_error.into_response()
    }
}

fn log_error_trace(status_code: StatusCode, trace: String) {
    if status_code.is_client_error() {
        tracing::error!("API WARNING: {:?}", trace);
    }
    if status_code.is_client_error() {
        tracing::error!("API ERROR: {:?}", trace);
    }
}
