use chrono::{DateTime, Utc};
use database::models::User;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Arguments of the `get_user` tool.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetUserParams {
    /// Identifier of the user to look up, as a UUID.
    pub user_id: Uuid,
}

/// Profile of a user, as returned by the `get_user` tool.
///
/// This is the tool's public contract; it deliberately omits `permissions`, which grants
/// rather than describes, and which no tool has a reason to hand to a model.
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetUserResult {
    pub id: Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for GetUserResult {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
