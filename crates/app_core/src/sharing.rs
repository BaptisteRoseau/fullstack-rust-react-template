//! Granting and revoking access to files and directories.
//!
//! Listing, granting and revoking all need [`PermissionLevel::Manager`] on the
//! resource. Two further rules apply to a grant:
//!
//! - nobody hands out more than they hold, so the level asked for is capped by
//!   the granter's own effective level;
//! - nobody grants to themselves, which would let a manager who inherited its
//!   level pin it down against the owner's wishes.

use database::Database;
use database::models::{DirectoryPermission, FilePermission};
use rbac::PermissionLevel;
use uuid::Uuid;

use crate::access::{self, ResourceRef};
use crate::error::CoreError;

/// Every grant on a file. Needs [`PermissionLevel::Manager`].
pub async fn list_file_permissions(
    db: &dyn Database,
    user_id: Uuid,
    file_id: Uuid,
) -> Result<Vec<FilePermission>, CoreError> {
    access::require(
        db,
        user_id,
        ResourceRef::File(file_id),
        PermissionLevel::Manager,
    )
    .await?;
    Ok(db.read_file_permissions(file_id).await?)
}

/// Grants `grantee` a level on a file, or changes the level it already holds.
/// Needs [`PermissionLevel::Manager`].
pub async fn grant_file_permission(
    db: &mut dyn Database,
    user_id: Uuid,
    file_id: Uuid,
    grantee: Uuid,
    level: PermissionLevel,
) -> Result<FilePermission, CoreError> {
    let resource = ResourceRef::File(file_id);
    check_grant(db, user_id, resource, grantee, level).await?;
    Ok(db
        .upsert_file_permission(file_id, grantee, level.as_str(), user_id)
        .await?)
}

/// Revokes whatever `grantee` was granted on a file. Needs
/// [`PermissionLevel::Manager`].
///
/// A grantee reaching the file through an ancestor directory keeps that access:
/// the inherited grant is a different row, revoked on the directory.
pub async fn revoke_file_permission(
    db: &mut dyn Database,
    user_id: Uuid,
    file_id: Uuid,
    grantee: Uuid,
) -> Result<(), CoreError> {
    access::require(
        db,
        user_id,
        ResourceRef::File(file_id),
        PermissionLevel::Manager,
    )
    .await?;

    if !db.delete_file_permission(file_id, grantee).await? {
        return Err(CoreError::NotFound(format!(
            "permission of {grantee} on {file_id}"
        )));
    }
    Ok(())
}

/// Every grant on a directory. Needs [`PermissionLevel::Manager`].
pub async fn list_directory_permissions(
    db: &dyn Database,
    user_id: Uuid,
    directory_id: Uuid,
) -> Result<Vec<DirectoryPermission>, CoreError> {
    access::require(
        db,
        user_id,
        ResourceRef::Directory(directory_id),
        PermissionLevel::Manager,
    )
    .await?;
    Ok(db.read_directory_permissions(directory_id).await?)
}

/// Grants `grantee` a level on a directory and, by inheritance, on everything
/// below it. Needs [`PermissionLevel::Manager`].
pub async fn grant_directory_permission(
    db: &mut dyn Database,
    user_id: Uuid,
    directory_id: Uuid,
    grantee: Uuid,
    level: PermissionLevel,
) -> Result<DirectoryPermission, CoreError> {
    let resource = ResourceRef::Directory(directory_id);
    check_grant(db, user_id, resource, grantee, level).await?;
    Ok(db
        .upsert_directory_permission(directory_id, grantee, level.as_str(), user_id)
        .await?)
}

/// Revokes whatever `grantee` was granted on a directory. Needs
/// [`PermissionLevel::Manager`].
pub async fn revoke_directory_permission(
    db: &mut dyn Database,
    user_id: Uuid,
    directory_id: Uuid,
    grantee: Uuid,
) -> Result<(), CoreError> {
    access::require(
        db,
        user_id,
        ResourceRef::Directory(directory_id),
        PermissionLevel::Manager,
    )
    .await?;

    if !db
        .delete_directory_permission(directory_id, grantee)
        .await?
    {
        return Err(CoreError::NotFound(format!(
            "permission of {grantee} on {directory_id}"
        )));
    }
    Ok(())
}

/// The three checks every grant passes: the granter manages the resource, the
/// grantee exists, and the level asked for does not exceed the granter's own.
async fn check_grant(
    db: &dyn Database,
    user_id: Uuid,
    resource: ResourceRef,
    grantee: Uuid,
    level: PermissionLevel,
) -> Result<(), CoreError> {
    access::require(db, user_id, resource, PermissionLevel::Manager).await?;

    if grantee == user_id {
        return Err(CoreError::InvalidRequest(
            "a user cannot grant a permission to itself".to_string(),
        ));
    }

    db.read_user(grantee)
        .await
        .map_err(|_| CoreError::NotFound(grantee.to_string()))?;

    let granter_level = access::effective_level(db, user_id, resource)
        .await?
        .ok_or_else(|| CoreError::NotFound(grantee.to_string()))?;

    if level > granter_level {
        return Err(CoreError::InvalidRequest(format!(
            "cannot grant {level}, which is above the granter's own {granter_level}"
        )));
    }
    Ok(())
}
