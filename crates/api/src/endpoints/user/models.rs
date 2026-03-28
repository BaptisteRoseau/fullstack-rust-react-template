use serde::Serialize;
use utoipa::{ToResponse, ToSchema};

/// Here is the documentation of the response
#[derive(Debug, Serialize, ToSchema, ToResponse)]
pub(crate) struct GetUserResponse {
    pub name: String,
}

impl From<String> for GetUserResponse {
    fn from(name: String) -> Self {
        Self { name }
    }
}
