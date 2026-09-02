use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToResponse, ToSchema};
use uuid::Uuid;

/// How much a user may do with a shared file or directory.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub(crate) enum PermissionLevel {
    /// List, download and preview.
    Viewer,
    /// Everything a viewer may do, plus uploading, renaming and creating children.
    Editor,
    /// Everything an editor may do, plus sharing, moving and deleting.
    Manager,
}

impl From<rbac::PermissionLevel> for PermissionLevel {
    fn from(level: rbac::PermissionLevel) -> Self {
        match level {
            rbac::PermissionLevel::Viewer => Self::Viewer,
            rbac::PermissionLevel::Editor => Self::Editor,
            rbac::PermissionLevel::Manager => Self::Manager,
        }
    }
}

impl From<PermissionLevel> for rbac::PermissionLevel {
    fn from(level: PermissionLevel) -> Self {
        match level {
            PermissionLevel::Viewer => Self::Viewer,
            PermissionLevel::Editor => Self::Editor,
            PermissionLevel::Manager => Self::Manager,
        }
    }
}

/// A directory in the caller's tree.
#[derive(Debug, Serialize, ToSchema, ToResponse)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetDirectoryResponse {
    pub id: Uuid,
    pub name: String,
    pub owner: Uuid,
    /// `null` when the directory sits at the root of its owner's tree.
    pub parent_id: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<database::models::Directory> for GetDirectoryResponse {
    fn from(directory: database::models::Directory) -> Self {
        Self {
            id: directory.id,
            name: directory.name,
            owner: directory.owner,
            parent_id: directory.parent_id,
            created_at: directory.created_at,
            updated_at: directory.updated_at,
        }
    }
}

/// A stored file. Sizes are reported both before and after the server's own
/// compression and encryption, so a client can show what the storage costs.
#[derive(Debug, Serialize, ToSchema, ToResponse)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetFileResponse {
    pub id: Uuid,
    pub name: String,
    pub owner: Uuid,
    /// `null` when the file sits at the root of its owner's tree.
    pub parent_id: Option<Uuid>,
    pub mime_type: String,
    /// Size of the original content, in bytes.
    pub size_bytes: i64,
    /// Size actually held by the object store, in bytes.
    pub stored_size_bytes: i64,
    /// Whether a thumbnail can be fetched for this file.
    pub has_thumbnail: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<database::models::File> for GetFileResponse {
    fn from(file: database::models::File) -> Self {
        Self {
            id: file.id,
            name: file.name,
            owner: file.owner,
            parent_id: file.parent_id,
            mime_type: file.mime_type,
            size_bytes: file.size_bytes,
            stored_size_bytes: file.stored_size_bytes,
            has_thumbnail: file.thumbnail_storage_key.is_some(),
            created_at: file.created_at,
            updated_at: file.updated_at,
        }
    }
}

/// One level of the tree: what the listed directory is, and what it holds.
#[derive(Debug, Serialize, ToSchema, ToResponse)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetEntriesResponse {
    /// The listed directory, or `null` when listing the caller's root.
    pub directory: Option<GetDirectoryResponse>,
    pub directories: Vec<GetDirectoryResponse>,
    pub files: Vec<GetFileResponse>,
}

/// Which directory to list. Omit `parentId` to list the caller's root.
#[derive(Debug, Deserialize, ToSchema, IntoParams)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetEntriesParams {
    pub parent_id: Option<Uuid>,
}

/// Where to place a newly uploaded file. Omit `parentId` to place it at the
/// caller's root.
#[derive(Debug, Deserialize, ToSchema, IntoParams)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PostUploadFileParams {
    pub parent_id: Option<Uuid>,
}

/// A new directory.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PostDirectoryRequest {
    pub name: String,
    /// Omit to create the directory at the caller's root.
    pub parent_id: Option<Uuid>,
}

/// A rename, a move, or both. An omitted field is left untouched; sending
/// `parentId: null` moves the entry to the caller's root.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PatchEntryRequest {
    pub name: Option<String>,
    #[schema(value_type = Option<Uuid>)]
    #[serde(default, deserialize_with = "deserialize_present")]
    pub parent_id: Option<Option<Uuid>>,
}

/// Tells an absent key from an explicit `null`, which mean different things
/// here: the first leaves the parent alone, the second moves the entry to the
/// root. `#[serde(default)]` covers the absent case, so reaching this function
/// at all already means the key was sent.
fn deserialize_present<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: Deserialize<'de>,
    D: serde::Deserializer<'de>,
{
    T::deserialize(deserializer).map(Some)
}

/// The level to grant.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PutPermissionRequest {
    pub level: PermissionLevel,
}

/// One grant on a file or directory.
#[derive(Debug, Serialize, ToSchema, ToResponse)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetPermissionResponse {
    pub id: Uuid,
    /// The user the level was granted to.
    pub grantee: Uuid,
    pub level: PermissionLevel,
    /// The user that granted it.
    pub granted_by: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl GetPermissionResponse {
    /// A level stored in a shape this enum does not know means the column and
    /// the code drifted apart; the row is reported at its least privileged
    /// reading rather than dropped, so a manager still sees the grant exists.
    fn new(
        id: Uuid,
        grantee: Uuid,
        stored_level: &str,
        granted_by: Uuid,
        created_at: chrono::DateTime<chrono::Utc>,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        let level = stored_level
            .parse::<rbac::PermissionLevel>()
            .unwrap_or(rbac::PermissionLevel::Viewer);
        Self {
            id,
            grantee,
            level: level.into(),
            granted_by,
            created_at,
            updated_at,
        }
    }
}

impl From<database::models::FilePermission> for GetPermissionResponse {
    fn from(permission: database::models::FilePermission) -> Self {
        Self::new(
            permission.id,
            permission.grantee,
            &permission.permission_level,
            permission.granted_by,
            permission.created_at,
            permission.updated_at,
        )
    }
}

impl From<database::models::DirectoryPermission> for GetPermissionResponse {
    fn from(permission: database::models::DirectoryPermission) -> Self {
        Self::new(
            permission.id,
            permission.grantee,
            &permission.permission_level,
            permission.granted_by,
            permission.created_at,
            permission.updated_at,
        )
    }
}
