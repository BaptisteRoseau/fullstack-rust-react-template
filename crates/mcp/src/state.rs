use std::sync::Arc;

use database::Database;
use tokio::sync::RwLock;

/// Everything the MCP tools are allowed to reach, injected once at startup.
///
/// The equivalent of `api`'s `AppState`, kept separate so this crate never depends on
/// `api`: a tool holds `Arc<RwLock<dyn Trait>>` and therefore never learns which backend
/// is behind it. Only the services a tool actually needs belong here — widening this
/// struct widens what an MCP client can reach, so add a field only with a tool that uses
/// it.
#[derive(Clone)]
pub struct McpState {
    pub(crate) database: Arc<RwLock<dyn Database>>,
}

impl McpState {
    pub fn new(database: Arc<RwLock<dyn Database>>) -> Self {
        Self { database }
    }
}
