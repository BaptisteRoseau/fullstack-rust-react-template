use rmcp::{tool, tool_router};

use crate::server::McpServer;

/// Liveness tools, the MCP counterpart of `api`'s `/ping` endpoint.
#[tool_router(router = ping_tool_router, vis = "pub(crate)")]
impl McpServer {
    /// Check that the backend is reachable and answering.
    #[tool(
        description = "Check that the backend is reachable and answering. Takes no \
argument and returns the literal string \"pong\"."
    )]
    async fn ping(&self) -> String {
        "pong".to_string()
    }
}
