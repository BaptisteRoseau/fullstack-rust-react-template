use crate::error::DatabaseError;
use crate::models::{File, NewFile};
use async_trait::async_trait;
use uuid::Uuid;

/// File rows. The blob itself lives in the storage backend, keyed by
/// [`File::storage_key`].
#[async_trait]
pub trait DatabaseFile: Send + Sync {
    async fn create_file(&mut self, file: NewFile) -> Result<File, Box<DatabaseError>>;

    async fn read_file(&self, id: Uuid) -> Result<File, Box<DatabaseError>>;

    /// The files directly inside `parent_id`, by name. A directory holding no
    /// file yields an empty list rather than [`DatabaseError::NotFound`].
    async fn read_files_by_parent(
        &self,
        parent_id: Uuid,
    ) -> Result<Vec<File>, Box<DatabaseError>>;

    /// The root-level files of `owner`, by name.
    async fn read_root_files(&self, owner: Uuid)
    -> Result<Vec<File>, Box<DatabaseError>>;

    /// Applies the fields that are `Some`. Passing `parent_id` as `Some(None)`
    /// moves the file to the root.
    async fn update_file(
        &mut self,
        id: Uuid,
        name: Option<String>,
        parent_id: Option<Option<Uuid>>,
    ) -> Result<File, Box<DatabaseError>>;

    /// Deletes the row only. The caller removes the stored blob.
    async fn delete_file(&mut self, id: Uuid) -> Result<bool, Box<DatabaseError>>;
}
