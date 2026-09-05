use rmcp::ErrorData;
use rmcp::model::{CallToolResult, ContentBlock};
use serde::Serialize;
use tracing::{error, warn};

use super::McpError;

/// Serialises a tool's output as the structured content of a successful result.
///
/// MCP clients read `structuredContent` as data and the text block as the human-readable
/// rendering of the same value; [`CallToolResult::structured`] fills both, so a tool only
/// has to hand over its result type.
pub(crate) fn structured<T: Serialize>(value: T) -> Result<CallToolResult, McpError> {
    Ok(CallToolResult::structured(serde_json::to_value(value)?))
}

/// Turns a tool's `Result` into the response the MCP client receives.
///
/// MCP separates two failure modes and only one of them reaches the model: a
/// [`CallToolResult::error`] is rendered by the client, whereas an [`ErrorData`] protocol
/// error is shown opaquely. Everything a [`McpError`] describes happened *inside* a tool
/// that ran and was routed correctly, so it is always the former — which is why the
/// returned `Result` is never `Err`.
///
/// Exactly like `api`'s `ApiError`, the detailed trace is logged server-side while the
/// caller only gets a short message.
pub(crate) fn into_tool_result(
    result: Result<CallToolResult, McpError>,
) -> Result<CallToolResult, ErrorData> {
    let error = match result {
        Ok(result) => return Ok(result),
        Err(error) => error,
    };

    let trace = format!("{error:?}");
    let message = match &error {
        McpError::NotFound(what) => format!("Not found: {what}"),
        McpError::CoreError(_) | McpError::SerializationError(_) => {
            "The backend could not serve this call.".to_string()
        }
    };

    match error {
        McpError::NotFound(_) => warn!(error = %trace, "MCP tool error"),
        McpError::CoreError(_) | McpError::SerializationError(_) => {
            error!(error = %trace, "MCP tool error")
        }
    }

    Ok(CallToolResult::error(vec![ContentBlock::text(message)]))
}

test_utils::tests_file!("_tests/test_response.rs");
