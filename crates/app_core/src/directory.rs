//! Directory tree operations: browsing, creating, renaming, moving, deleting.
//!
//! Every function that touches an existing resource goes through
//! [`access::require`] first, so an unauthorised caller is answered
//! [`CoreError::NotFound`] before any work happens.

use database::Database;
use database::models::{Directory, File};
use rbac::PermissionLevel;
use uuid::Uuid;

use crate::access::{self, ResourceRef};
use crate::error::CoreError;

/// Longest name the `directories.name` and `files.name` columns hold.
pub const MAX_NAME_LENGTH: usize = 255;

/// One level of the tree: the directory being looked at, if it is not the
/// caller's root, and its direct children.
#[derive(Debug)]
pub struct DirectoryListing {
    /// `None` when listing the caller's root, which is not a row of its own.
    pub directory: Option<Directory>,
    pub directories: Vec<Directory>,
    pub files: Vec<File>,
}

/// Lists the direct children of `parent_id`, or of the caller's root when it is
/// `None`. Needs [`PermissionLevel::Viewer`] on the directory.
pub async fn list_entries(
    db: &dyn Database,
    user_id: Uuid,
    parent_id: Option<Uuid>,
) -> Result<DirectoryListing, CoreError> {
    let Some(parent_id) = parent_id else {
        return Ok(DirectoryListing {
            directory: None,
            directories: db.read_root_directories(user_id).await?,
            files: db.read_root_files(user_id).await?,
        });
    };

    access::require(
        db,
        user_id,
        ResourceRef::Directory(parent_id),
        PermissionLevel::Viewer,
    )
    .await?;

    Ok(DirectoryListing {
        directory: Some(db.read_directory(parent_id).await?),
        directories: db.read_child_directories(parent_id).await?,
        files: db.read_files_by_parent(parent_id).await?,
    })
}

/// Reads one directory's metadata. Needs [`PermissionLevel::Viewer`].
pub async fn read_directory(
    db: &dyn Database,
    user_id: Uuid,
    id: Uuid,
) -> Result<Directory, CoreError> {
    access::require(
        db,
        user_id,
        ResourceRef::Directory(id),
        PermissionLevel::Viewer,
    )
    .await?;
    Ok(db.read_directory(id).await?)
}

/// Creates a directory under `parent_id`, or at the caller's root when it is
/// `None`. Needs [`PermissionLevel::Editor`] on the parent.
pub async fn create_directory(
    db: &mut dyn Database,
    user_id: Uuid,
    name: String,
    parent_id: Option<Uuid>,
) -> Result<Directory, CoreError> {
    let name = validate_name(name)?;
    if let Some(parent_id) = parent_id {
        access::require(
            db,
            user_id,
            ResourceRef::Directory(parent_id),
            PermissionLevel::Editor,
        )
        .await?;
    }
    Ok(db.create_directory(user_id, parent_id, name).await?)
}

/// Renames and/or moves a directory. Needs [`PermissionLevel::Editor`] on it,
/// and on the destination when it moves.
///
/// Passing `parent_id` as `Some(None)` moves the directory to the caller's
/// root, which only its owner may do — a root belongs to one user, so nobody
/// else can place anything in it.
pub async fn update_directory(
    db: &mut dyn Database,
    user_id: Uuid,
    id: Uuid,
    name: Option<String>,
    parent_id: Option<Option<Uuid>>,
) -> Result<Directory, CoreError> {
    access::require(
        db,
        user_id,
        ResourceRef::Directory(id),
        PermissionLevel::Editor,
    )
    .await?;

    let name = name.map(validate_name).transpose()?;

    if let Some(destination) = parent_id {
        check_move_destination(db, user_id, destination, Some(id)).await?;
        if db.read_directory(id).await?.owner != user_id && destination.is_none() {
            return Err(CoreError::InvalidRequest(
                "only the owner may move a directory to the root".to_string(),
            ));
        }
    }

    Ok(db.update_directory(id, name, parent_id).await?)
}

/// Deletes a directory and, through the database's cascades, everything below
/// it. Needs [`PermissionLevel::Manager`].
///
/// The blobs of the files it held are **not** removed: this deletes the tree in
/// one statement and never learns which storage keys went with it. Reclaiming
/// them is a sweep over storage keys with no row, which the storage layer does
/// not offer yet.
pub async fn delete_directory(
    db: &mut dyn Database,
    user_id: Uuid,
    id: Uuid,
) -> Result<(), CoreError> {
    access::require(
        db,
        user_id,
        ResourceRef::Directory(id),
        PermissionLevel::Manager,
    )
    .await?;

    if !db.delete_directory(id).await? {
        return Err(CoreError::NotFound(id.to_string()));
    }
    Ok(())
}

/// Holds a move to a destination the caller may write into, and — when a
/// directory is being moved — refuses to place it inside its own subtree,
/// which would detach that subtree from every root.
pub(crate) async fn check_move_destination(
    db: &dyn Database,
    user_id: Uuid,
    destination: Option<Uuid>,
    moved_directory: Option<Uuid>,
) -> Result<(), CoreError> {
    let Some(destination) = destination else {
        return Ok(());
    };

    access::require(
        db,
        user_id,
        ResourceRef::Directory(destination),
        PermissionLevel::Editor,
    )
    .await?;

    let Some(moved_directory) = moved_directory else {
        return Ok(());
    };

    let ancestors = db.read_directory_ancestors(destination).await?;
    if ancestors
        .iter()
        .any(|directory| directory.id == moved_directory)
    {
        return Err(CoreError::InvalidRequest(
            "a directory cannot be moved inside itself".to_string(),
        ));
    }
    Ok(())
}

/// Trims the name and holds it to what the column accepts, so a rejected name
/// fails here rather than as an opaque database error.
pub(crate) fn validate_name(name: String) -> Result<String, CoreError> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err(CoreError::InvalidRequest("the name is empty".to_string()));
    }
    if trimmed.chars().count() > MAX_NAME_LENGTH {
        return Err(CoreError::InvalidRequest(format!(
            "the name is longer than {MAX_NAME_LENGTH} characters"
        )));
    }
    if trimmed.contains('/') || trimmed.contains('\\') {
        return Err(CoreError::InvalidRequest(
            "the name cannot contain a path separator".to_string(),
        ));
    }
    Ok(trimmed.to_string())
}

test_utils::tests_file!("_tests/test_directory.rs");
