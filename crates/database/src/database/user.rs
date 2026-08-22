use crate::models::{User, UserPatch};

use crate::error::DatabaseError;
use async_trait::async_trait;
use uuid::Uuid;

/// Defines the Database trait interface.
#[async_trait]
pub trait DatabaseUser: Send + Sync {
    async fn create_user(&mut self, patch: UserPatch)
    -> Result<User, Box<DatabaseError>>;

    /// Applies the fields the patch sets. Answers [`DatabaseError::NotFound`]
    /// when no row carries the patch's id.
    async fn update_user(&mut self, patch: UserPatch)
    -> Result<User, Box<DatabaseError>>;
    async fn read_user(&self, uuid: Uuid) -> Result<User, Box<DatabaseError>>;
    async fn delete_user(&mut self, uuid: Uuid) -> Result<bool, Box<DatabaseError>>;

    /// Creates the row on first login from the identity provider's claims.
    ///
    /// On later logins only `username` and `email` are re-synced: `first_name`
    /// and `last_name` become locally owned once the row exists, so that a
    /// profile edit is not reverted by the next login.
    async fn register(
        &mut self,
        id: Uuid,
        username: String,
        first_name: String,
        last_name: String,
        email: String,
    ) -> Result<User, Box<DatabaseError>>;
}
