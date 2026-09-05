use rmcp::ErrorData;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::CallToolResult;
use rmcp::{tool, tool_router};
use uuid::Uuid;

use super::models::{GetUserParams, GetUserResult};
use crate::error::{McpError, into_tool_result, structured};
use crate::server::McpServer;

/// User lookups, the MCP counterpart of `api`'s `/user` endpoints.
#[tool_router(router = user_tool_router, vis = "pub(crate)")]
impl McpServer {
    /// Read the profile of one user.
    #[tool(
        description = "Read the stored profile of one user, given their UUID. Returns \
their username, name, email and timestamps, and fails when no such user has logged in."
    )]
    async fn get_user(
        &self,
        Parameters(GetUserParams { user_id }): Parameters<GetUserParams>,
    ) -> Result<CallToolResult, ErrorData> {
        into_tool_result(self.read_user(user_id).await)
    }

    /// Body of [`Self::get_user`], split out so it can use `?` over [`McpError`].
    ///
    /// A tool is the MCP equivalent of an `api` handler: it takes its arguments, calls
    /// [`app_core`], and serialises the result. Domain rules never live here, and the
    /// lock window stays as narrow as the call that needs it.
    async fn read_user(&self, user_id: Uuid) -> Result<CallToolResult, McpError> {
        let profile = {
            let database = self.state.database.read().await;
            app_core::user::read_profile(&*database, user_id).await?
        };

        let profile = profile.ok_or_else(|| McpError::NotFound(user_id.to_string()))?;
        structured(GetUserResult::from(profile))
    }
}
