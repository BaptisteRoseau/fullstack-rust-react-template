use crate::models::ApiKey;

use crate::error::DatabaseError;
use async_trait::async_trait;
use uuid::Uuid;

/// Defines the Database trait interface.
#[async_trait]
pub trait DatabaseApiKey: Send + Sync {
    async fn create_api_key(
        &mut self,
        owner: Uuid,
        name: String,
        hash: String,
        permissions: serde_json::Value,
    ) -> Result<ApiKey, Box<DatabaseError>>;

    async fn read_api_key_by_id(&self, id: Uuid) -> Result<ApiKey, Box<DatabaseError>>;
    async fn read_api_key_by_hash(
        &self,
        hash: &str,
    ) -> Result<ApiKey, Box<DatabaseError>>;

    /// Every key owned by `owner`, newest first. An owner with no key yields an
    /// empty list rather than [`DatabaseError::NotFound`].
    async fn read_api_keys_by_owner(
        &self,
        owner: Uuid,
    ) -> Result<Vec<ApiKey>, Box<DatabaseError>>;

    async fn delete_api_key(&mut self, id: Uuid) -> Result<bool, Box<DatabaseError>>;
}
