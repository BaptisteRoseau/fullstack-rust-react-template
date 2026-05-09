use serde::{Deserialize, Serialize};
use utoipa::{ToResponse, ToSchema};
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CreateApiKeyRequest {
    pub name: String,
    pub permissions: Vec<String>,
}

#[derive(Debug, Serialize, ToSchema, ToResponse)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CreateApiKeyResponse {
    pub id: Uuid,
    pub name: String,
    pub key: String,
    pub permissions: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl CreateApiKeyResponse {
    pub fn new(raw_key: String, api_key: models::ApiKey) -> Self {
        Self {
            id: api_key.id,
            name: api_key.name,
            key: raw_key,
            permissions: api_key
                .permissions
                .iter()
                .map(|p| format!("{p:?}"))
                .collect(),
            created_at: api_key.created_at,
        }
    }
}

#[derive(Debug, Serialize, ToSchema, ToResponse)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetApiKeyResponse {
    pub id: Uuid,
    pub name: String,
    pub permissions: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<models::ApiKey> for GetApiKeyResponse {
    fn from(key: models::ApiKey) -> Self {
        Self {
            id: key.id,
            name: key.name,
            permissions: key.permissions.iter().map(|p| format!("{p:?}")).collect(),
            created_at: key.created_at,
        }
    }
}
