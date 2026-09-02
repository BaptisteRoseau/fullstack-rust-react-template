use crate::error::DatabaseError;
use crate::models::FilePermission;
use async_trait::async_trait;
use uuid::Uuid;

/// Grants on individual files. A file is also reachable through a grant on any
/// of its ancestor directories; that half lives in
/// [`DatabaseDirectoryPermission`](super::DatabaseDirectoryPermission).
#[async_trait]
pub trait DatabaseFilePermission: Send + Sync {
    /// Creates the grant, or replaces the level of the one already held by
    /// `grantee` on `file_id`.
    async fn upsert_file_permission(
        &mut self,
        file_id: Uuid,
        grantee: Uuid,
        permission_level: &str,
        granted_by: Uuid,
    ) -> Result<FilePermission, Box<DatabaseError>>;

    /// The grant `grantee` holds on `file_id`, or `None`.
    async fn read_file_permission(
        &self,
        file_id: Uuid,
        grantee: Uuid,
    ) -> Result<Option<FilePermission>, Box<DatabaseError>>;

    /// Every grant on `file_id`, oldest first.
    async fn read_file_permissions(
        &self,
        file_id: Uuid,
    ) -> Result<Vec<FilePermission>, Box<DatabaseError>>;

    async fn delete_file_permission(
        &mut self,
        file_id: Uuid,
        grantee: Uuid,
    ) -> Result<bool, Box<DatabaseError>>;
}
