use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToResponse, ToSchema};
use uuid::Uuid;

/// Here is the documentation of the response
#[derive(Debug, Serialize, ToSchema, ToResponse)]
pub(crate) struct GetUserResponse {
    pub name: String,
}

impl From<String> for GetUserResponse {
    fn from(value: String) -> Self {
        Self { name: value }
    }
}

/// Here is the documentation of the parameter
#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub(crate) struct PostUserParams {
    pub id: Uuid,
}
