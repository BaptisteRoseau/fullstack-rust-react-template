use axum::http::request::Parts;
use rbac::UserPermissions;

use crate::error::McpError;

/// The caller `api` stored in the request extensions after checking the credential.
pub(crate) fn caller(parts: &Parts) -> Result<&UserPermissions, McpError> {
    parts
        .extensions
        .get::<UserPermissions>()
        .ok_or(McpError::MissingCaller)
}
