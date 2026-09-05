use app_core::error::CoreError;

/// Everything a tool of this crate can fail with.
///
/// Deliberately crate-private: it never crosses the crate boundary, because MCP carries a
/// tool's failure inside the protocol response rather than in a Rust error. See
/// [`super::response`] for the conversion.
#[derive(Debug, thiserror::Error)]
pub(crate) enum McpError {
    #[error("Not found: {0}")]
    NotFound(String),
    #[error(transparent)]
    CoreError(#[from] CoreError),
    #[error("Could not serialize the tool result: {0}")]
    SerializationError(#[from] serde_json::Error),
}
