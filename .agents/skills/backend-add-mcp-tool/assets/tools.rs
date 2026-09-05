//! The tools of this group.
//!
//! An inherent `impl McpServer` block: Rust allows one to be split across modules, which
//! is what lets each group own its file while all of them attach to the same server.
//!
//! Remember to sum `widget_tool_router` into the `#[tool_handler]` attribute in
//! `crates/mcp/src/server.rs` — a tool that is not summed there does not exist.

use rmcp::ErrorData;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::CallToolResult;
use rmcp::{tool, tool_router};
use uuid::Uuid;

use super::models::{GetWidgetParams, GetWidgetResult};
use crate::error::{McpError, into_tool_result, structured};
use crate::server::McpServer;

/// Widget lookups.
#[tool_router(router = widget_tool_router, vis = "pub(crate)")]
impl McpServer {
    /// Read one widget.
    #[tool(description = "Read one widget by its UUID. Returns its name, and fails when \
no widget has that id.")]
    async fn get_widget(
        &self,
        Parameters(GetWidgetParams { widget_id }): Parameters<GetWidgetParams>,
    ) -> Result<CallToolResult, ErrorData> {
        into_tool_result(self.read_widget(widget_id).await)
    }

    /// Body of [`Self::get_widget`], split out so it can use `?` over [`McpError`].
    ///
    /// Takes the arguments, calls `app_core`, serialises the result. No domain logic, and
    /// the lock window stays as narrow as the call that needs it.
    async fn read_widget(&self, widget_id: Uuid) -> Result<CallToolResult, McpError> {
        let widget = {
            let database = self.state.database.read().await;
            app_core::widget::get_widget(&*database, widget_id).await?
        };

        structured(GetWidgetResult::from(widget))
    }
}
