// Re-export generated models
pub use crate::generated_models::*;

use chrono::{DateTime, Utc};
use uuid::Uuid;

/* ======================================================================================
CLOUD STORAGE ROWS

Hand-written rather than generated: `database_crud_derive::Crud` cannot express a
nullable `Option<Uuid>` foreign key, which `parent_id` needs to mark a root-level
entry, nor the tree and permission queries these tables are read through.
====================================================================================== */

/// A directory in a user's tree. `parent_id` is `None` at the root.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Directory {
    pub id: Uuid,
    pub owner: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A stored file. The blob it points at is compressed then encrypted, so
/// `stored_size_bytes` counts ciphertext while `size_bytes` counts the original.
///
/// `encrypted_dek` is the per-file data encryption key, itself encrypted under
/// the server master key with `dek_nonce`. The content is encrypted with that
/// key and `content_nonce`; the thumbnail, when present, reuses the same key
/// with `thumbnail_nonce`.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct File {
    pub id: Uuid,
    pub owner: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub storage_key: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub stored_size_bytes: i64,
    pub is_compressed: bool,
    pub encrypted_dek: Vec<u8>,
    pub dek_nonce: Vec<u8>,
    pub content_nonce: Vec<u8>,
    pub thumbnail_storage_key: Option<String>,
    pub thumbnail_nonce: Option<Vec<u8>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Everything an insert into `files` needs, gathered so the trait method keeps
/// one parameter instead of fourteen.
///
/// The `id` is chosen by the caller rather than defaulted by the column: the
/// storage key is derived from it, so it has to be known before the blob is
/// written.
#[derive(Debug, Clone)]
pub struct NewFile {
    pub id: Uuid,
    pub owner: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub storage_key: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub stored_size_bytes: i64,
    pub is_compressed: bool,
    pub encrypted_dek: Vec<u8>,
    pub dek_nonce: Vec<u8>,
    pub content_nonce: Vec<u8>,
    pub thumbnail_storage_key: Option<String>,
    pub thumbnail_nonce: Option<Vec<u8>>,
}

/// An explicit grant on one directory. It also covers everything below that
/// directory, so the inheritance walk reads these rows for every ancestor.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DirectoryPermission {
    pub id: Uuid,
    pub directory_id: Uuid,
    pub grantee: Uuid,
    pub permission_level: String,
    pub granted_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// An explicit grant on one file.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct FilePermission {
    pub id: Uuid,
    pub file_id: Uuid,
    pub grantee: Uuid,
    pub permission_level: String,
    pub granted_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
