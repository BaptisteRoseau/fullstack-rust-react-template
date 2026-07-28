use crate::models::{ApiKey, User, UserPatch};

use super::error::DatabaseError;
use async_trait::async_trait;
use uuid::Uuid;

/// Defines the Database trait interface.
#[async_trait]
pub trait Database: Send + Sync {
    async fn create_user(&mut self, patch: UserPatch)
    -> Result<User, Box<DatabaseError>>;
    async fn update_user(&mut self, patch: UserPatch)
    -> Result<User, Box<DatabaseError>>;
    async fn read_user(&self, uuid: Uuid) -> Result<User, Box<DatabaseError>>;
    async fn delete_user(&mut self, uuid: Uuid) -> Result<bool, Box<DatabaseError>>;

    /// Registers a user from OIDC identity claims: creates the row if `id` is
    /// unknown, otherwise updates it if any field drifted from the identity
    /// provider's current values.
    async fn register(
        &mut self,
        id: Uuid,
        username: String,
        first_name: String,
        last_name: String,
        email: String,
    ) -> Result<User, Box<DatabaseError>>;

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

    async fn delete_api_key(&mut self, id: Uuid) -> Result<bool, Box<DatabaseError>>;
}
