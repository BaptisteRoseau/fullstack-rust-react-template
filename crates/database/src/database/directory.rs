use crate::error::DatabaseError;
use crate::models::Directory;
use async_trait::async_trait;
use uuid::Uuid;

/// Directory rows and the tree walks the access check needs.
#[async_trait]
pub trait DatabaseDirectory: Send + Sync {
    /// Creates a directory. `parent_id` is `None` for a root-level directory.
    async fn create_directory(
        &mut self,
        owner: Uuid,
        parent_id: Option<Uuid>,
        name: String,
    ) -> Result<Directory, Box<DatabaseError>>;

    async fn read_directory(&self, id: Uuid) -> Result<Directory, Box<DatabaseError>>;

    /// Direct sub-directories of `parent_id`, by name. A directory with no child
    /// yields an empty list rather than [`DatabaseError::NotFound`].
    async fn read_child_directories(
        &self,
        parent_id: Uuid,
    ) -> Result<Vec<Directory>, Box<DatabaseError>>;

    /// The root-level directories of `owner`, by name.
    async fn read_root_directories(
        &self,
        owner: Uuid,
    ) -> Result<Vec<Directory>, Box<DatabaseError>>;

    /// The directory carrying `id` followed by every ancestor up to the root,
    /// closest first.
    ///
    /// Answers an empty list when no directory carries `id`, so a caller
    /// walking a deleted branch sees no grant rather than an error.
    async fn read_directory_ancestors(
        &self,
        id: Uuid,
    ) -> Result<Vec<Directory>, Box<DatabaseError>>;

    /// Applies the fields that are `Some`. Passing `parent_id` as
    /// `Some(None)` moves the directory to the root.
    async fn update_directory(
        &mut self,
        id: Uuid,
        name: Option<String>,
        parent_id: Option<Option<Uuid>>,
    ) -> Result<Directory, Box<DatabaseError>>;

    /// Deletes the directory. Children and grants go with it through the
    /// `ON DELETE CASCADE` foreign keys.
    async fn delete_directory(&mut self, id: Uuid) -> Result<bool, Box<DatabaseError>>;
}
