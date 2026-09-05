use std::sync::Arc;

use axum::Router;
use config::McpConfig;
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use rmcp::transport::streamable_http_server::{
    StreamableHttpServerConfig, StreamableHttpService,
};

use crate::server::McpServer;
use crate::state::McpState;

/// Builds the router holding the MCP Streamable HTTP endpoint.
///
/// This function and [`McpState`] are the entire public surface of this crate: the
/// handler, its tools and `rmcp` itself stay invisible to the caller, which only mounts
/// the returned `Router` into its own. There is no stdio transport — the backend is a
/// server, and its MCP endpoint is reached over HTTP like every other one.
///
/// The router is generic over the caller's state, like `api`'s `with_middlewares`, so it
/// can be merged into a `Router<AppState>` that has not been given its state yet. The
/// endpoint itself needs no Axum state: everything its tools use is captured from
/// `state`.
pub fn mcp_routes<S>(config: &McpConfig, state: McpState) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    // The transport asks for a fresh handler per request, so the closure clones the
    // captured state rather than moving it. Sessions are off: the tools hold no
    // per-connection state, and a stateless endpoint answers a plain JSON body instead
    // of an event stream, which survives the API's request timeout unchanged.
    let service = StreamableHttpService::new(
        move || Ok(McpServer::new(state.clone())),
        Arc::new(LocalSessionManager::default()),
        StreamableHttpServerConfig::default()
            .with_legacy_session_mode(false)
            .with_json_response(config.json_response)
            .with_allowed_hosts(config.allowed_hosts.clone()),
    );

    // `route_service` binds every method on the path, which the protocol needs: a client
    // POSTs its calls, and may GET or DELETE the same URL.
    Router::new().route_service(&config.path, service)
}

test_utils::tests_file!("_tests/test_routes.rs");
