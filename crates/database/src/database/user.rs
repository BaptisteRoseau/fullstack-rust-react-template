use crate::models::{User, UserPatch};

use crate::error::DatabaseError;
use async_trait::async_trait;
use uuid::Uuid;

/// Defines the Database trait interface.
#[async_trait]
pub trait DatabaseUser: Send + Sync {
    async fn create_user(&mut self, patch: UserPatch)
    -> Result<User, Box<DatabaseError>>;
    async fn update_user(&mut self, patch: UserPatch)
    -> Result<User, Box<DatabaseError>>;
    async fn read_user(&self, uuid: Uuid) -> Result<User, Box<DatabaseError>>;
    async fn delete_user(&mut self, uuid: Uuid) -> Result<bool, Box<DatabaseError>>;
    async fn register(
        &mut self,
        id: Uuid,
        username: String,
        first_name: String,
        last_name: String,
        email: String,
    ) -> Result<User, Box<DatabaseError>>;
}
