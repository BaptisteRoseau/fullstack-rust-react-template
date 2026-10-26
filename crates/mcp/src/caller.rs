use authenticator::Caller;
use axum::http::request::Parts;

use crate::error::McpError;

/// The caller `api` stored in the request extensions after checking the credential.
pub(crate) fn caller(parts: &Parts) -> Result<&Caller, McpError> {
    parts
        .extensions
        .get::<Caller>()
        .ok_or(McpError::MissingCaller)
}
