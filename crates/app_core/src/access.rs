//! Who may do what to a directory or a file.
//!
//! Three rules, applied together:
//!
//! 1. The owner of a resource always holds [`PermissionLevel::Manager`] and
//!    never needs a row in the permission tables.
//! 2. An explicit grant on the resource itself applies.
//! 3. A grant on **any ancestor directory** applies too, and so does owning one
//!    — sharing a directory shares everything below it, the way a shared folder
//!    behaves in a consumer drive.
//!
//! The effective level is the highest of whatever applies. Callers ask for a
//! minimum through [`require`], which answers [`CoreError::NotFound`] rather
//! than a distinct "forbidden" so that a caller cannot probe for the existence
//! of a resource it may not see.

use database::Database;
use rbac::PermissionLevel;
use uuid::Uuid;

use crate::error::CoreError;

/// The resource an access question is about.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceRef {
    Directory(Uuid),
    File(Uuid),
}

impl ResourceRef {
    fn id(&self) -> Uuid {
        match self {
            Self::Directory(id) | Self::File(id) => *id,
        }
    }
}

/// The highest level `user_id` holds on `resource`, or `None` when it holds
/// none at all.
///
/// Answers `None` — not an error — when the resource does not exist, so that a
/// caller cannot tell "gone" from "not mine".
pub async fn effective_level(
    db: &dyn Database,
    user_id: Uuid,
    resource: ResourceRef,
) -> Result<Option<PermissionLevel>, CoreError> {
    match resource {
        ResourceRef::Directory(id) => directory_level(db, user_id, id).await,
        ResourceRef::File(id) => file_level(db, user_id, id).await,
    }
}

/// Answers whether `user_id` holds at least `minimum` on `resource`.
pub async fn has_access(
    db: &dyn Database,
    user_id: Uuid,
    resource: ResourceRef,
    minimum: PermissionLevel,
) -> Result<bool, CoreError> {
    Ok(effective_level(db, user_id, resource)
        .await?
        .is_some_and(|level| level >= minimum))
}

/// Guard for the top of every operation on an existing resource.
///
/// Answers [`CoreError::NotFound`] both when the resource does not exist and
/// when the caller may not reach it at that level, so a rejected caller learns
/// nothing about what exists.
pub async fn require(
    db: &dyn Database,
    user_id: Uuid,
    resource: ResourceRef,
    minimum: PermissionLevel,
) -> Result<(), CoreError> {
    if has_access(db, user_id, resource, minimum).await? {
        return Ok(());
    }
    Err(CoreError::NotFound(resource.id().to_string()))
}

async fn directory_level(
    db: &dyn Database,
    user_id: Uuid,
    directory_id: Uuid,
) -> Result<Option<PermissionLevel>, CoreError> {
    let ancestors = db.read_directory_ancestors(directory_id).await?;
    if ancestors.is_empty() {
        return Ok(None);
    }
    inherited_level(db, user_id, &ancestors).await
}

async fn file_level(
    db: &dyn Database,
    user_id: Uuid,
    file_id: Uuid,
) -> Result<Option<PermissionLevel>, CoreError> {
    let Ok(file) = db.read_file(file_id).await else {
        return Ok(None);
    };
    if file.owner == user_id {
        return Ok(Some(PermissionLevel::Manager));
    }

    let own_grant = db
        .read_file_permission(file_id, user_id)
        .await?
        .map(|permission| parse_level(&permission.permission_level))
        .transpose()?;

    let from_tree = match file.parent_id {
        None => None,
        Some(parent_id) => {
            let ancestors = db.read_directory_ancestors(parent_id).await?;
            inherited_level(db, user_id, &ancestors).await?
        }
    };

    Ok(highest(own_grant, from_tree))
}

/// The level `user_id` draws from a directory chain: `Manager` as soon as it
/// owns one of them, otherwise the highest grant it holds among them.
async fn inherited_level(
    db: &dyn Database,
    user_id: Uuid,
    ancestors: &[database::models::Directory],
) -> Result<Option<PermissionLevel>, CoreError> {
    if ancestors.iter().any(|directory| directory.owner == user_id) {
        return Ok(Some(PermissionLevel::Manager));
    }

    let ids: Vec<Uuid> = ancestors.iter().map(|directory| directory.id).collect();
    let grants = db
        .read_directory_permissions_for_grantee(&ids, user_id)
        .await?;

    let mut best = None;
    for grant in grants {
        best = highest(best, Some(parse_level(&grant.permission_level)?));
    }
    Ok(best)
}

fn highest(
    left: Option<PermissionLevel>,
    right: Option<PermissionLevel>,
) -> Option<PermissionLevel> {
    match (left, right) {
        (Some(left), Some(right)) => Some(left.max(right)),
        (found, None) | (None, found) => found,
    }
}

/// A stored level that no longer parses means the column and this enum drifted
/// apart; that is a server fault, not a caller mistake.
fn parse_level(stored: &str) -> Result<PermissionLevel, CoreError> {
    stored.parse().map_err(|_| {
        CoreError::InvalidRequest(format!("unknown permission level: {stored}"))
    })
}

test_utils::tests_file!("_tests/test_access.rs");
