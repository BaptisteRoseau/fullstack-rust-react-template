use axum::http::request::Parts;
use rmcp::ErrorData;
use rmcp::handler::server::tool::Extension;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::CallToolResult;
use rmcp::{tool, tool_router};
use uuid::Uuid;

use super::models::{GetUserParams, GetUserResult};
use crate::caller::caller;
use crate::error::{McpError, into_tool_result, structured};
use crate::server::McpServer;

/// User lookups, the MCP counterpart of `api`'s `/user` endpoints.
#[tool_router(router = user_tool_router, vis = "pub(crate)")]
impl McpServer {
    /// Read the profile of one user.
    #[tool(
        description = "Read the stored profile of one user, given their UUID. Returns \
their username, name, email and timestamps. Only the caller's own profile can be read; \
any other UUID fails as not found."
    )]
    async fn get_user(
        &self,
        Extension(parts): Extension<Parts>,
        Parameters(GetUserParams { user_id }): Parameters<GetUserParams>,
    ) -> Result<CallToolResult, ErrorData> {
        into_tool_result(self.read_user(&parts, user_id).await)
    }

    /// Body of [`Self::get_user`], split out so it can use `?` over [`McpError`].
    async fn read_user(
        &self,
        parts: &Parts,
        user_id: Uuid,
    ) -> Result<CallToolResult, McpError> {
        let caller = caller(parts)?;
        let profile = {
            let database = self.state.database.read().await;
            app_core::user::read_profile(&*database, caller, user_id).await?
        };

        let profile = profile.ok_or_else(|| McpError::NotFound(user_id.to_string()))?;
        structured(GetUserResult::from(profile))
    }
}
