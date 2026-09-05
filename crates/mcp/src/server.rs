use rmcp::ServerHandler;
use rmcp::model::{Implementation, ServerCapabilities, ServerInfo};
use rmcp::tool_handler;

use crate::state::McpState;

/// The server name advertised to MCP clients, shown next to its tools.
const SERVER_NAME: &str = "backend";

/// What the model is told the server is for, before it sees any tool.
const SERVER_INSTRUCTIONS: &str = "Tools exposed by the backend of this application. \
Every tool answers from the application's own data; none of them reach the public internet.";

/// The MCP server: one struct holding the state, with the tools attached to it.
///
/// The tools themselves live under [`crate::tools`], one directory per group, each
/// contributing an inherent `impl McpServer` block whose `#[tool_router]` builds a
/// [`ToolRouter`]. Those routers are summed in the [`tool_handler`] attribute below,
/// which is the MCP counterpart of `api`'s `routes/router.rs`: the one list deciding what
/// the outside world can call. A tool that is written but not summed here does not exist.
///
/// [`ToolRouter`]: rmcp::handler::server::router::tool::ToolRouter
#[derive(Clone)]
pub(crate) struct McpServer {
    pub(crate) state: McpState,
}

impl McpServer {
    pub(crate) fn new(state: McpState) -> Self {
        Self { state }
    }
}

// The parentheses are load-bearing: the macro pastes this expression in front of a method
// call, and a bare `a() + b()` would bind that call to the right operand only.
#[tool_handler(router = (Self::ping_tool_router() + Self::user_tool_router()))]
impl ServerHandler for McpServer {
    /// Metadata sent back to the client during the MCP handshake.
    ///
    /// Written out rather than left to the macro so the name and the instructions stay
    /// next to each other as named constants, the way `api`'s `api_info` does.
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new(SERVER_NAME, env!("CARGO_PKG_VERSION")))
            .with_instructions(SERVER_INSTRUCTIONS.to_string())
    }
}
