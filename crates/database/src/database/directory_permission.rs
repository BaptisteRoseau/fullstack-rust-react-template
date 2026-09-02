use crate::error::DatabaseError;
use crate::models::DirectoryPermission;
use async_trait::async_trait;
use uuid::Uuid;

/// Grants on directories. A grant covers the directory and everything below it,
/// which is why the effective-access check reads them for a whole ancestor chain
/// at once through [`Self::read_directory_permissions_for_grantee`].
#[async_trait]
pub trait DatabaseDirectoryPermission: Send + Sync {
    /// Creates the grant, or replaces the level of the one already held by
    /// `grantee` on `directory_id`.
    async fn upsert_directory_permission(
        &mut self,
        directory_id: Uuid,
        grantee: Uuid,
        permission_level: &str,
        granted_by: Uuid,
    ) -> Result<DirectoryPermission, Box<DatabaseError>>;

    /// The grant `grantee` holds on `directory_id`, or `None`.
    async fn read_directory_permission(
        &self,
        directory_id: Uuid,
        grantee: Uuid,
    ) -> Result<Option<DirectoryPermission>, Box<DatabaseError>>;

    /// Every grant `grantee` holds among `directory_ids`, in one round trip.
    async fn read_directory_permissions_for_grantee(
        &self,
        directory_ids: &[Uuid],
        grantee: Uuid,
    ) -> Result<Vec<DirectoryPermission>, Box<DatabaseError>>;

    /// Every grant on `directory_id`, oldest first.
    async fn read_directory_permissions(
        &self,
        directory_id: Uuid,
    ) -> Result<Vec<DirectoryPermission>, Box<DatabaseError>>;

    async fn delete_directory_permission(
        &mut self,
        directory_id: Uuid,
        grantee: Uuid,
    ) -> Result<bool, Box<DatabaseError>>;
}
