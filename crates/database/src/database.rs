use crate::models::{User, UserPatch};

/**
* Defines the Database trait interface.
**/
use super::error::DatabaseError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Database: Send + Sync {
    async fn create_user(&mut self, patch: UserPatch) -> Result<User, Box<DatabaseError>>;
    async fn update_user(&mut self, patch: UserPatch) -> Result<User, Box<DatabaseError>>;
    async fn read_user(&self, uuid: Uuid) -> Result<User, Box<DatabaseError>>;
    async fn delete_user(&mut self, uuid: Uuid) -> Result<(), Box<DatabaseError>>;
}
