use async_trait::async_trait;
use std::path::Path;

use crate::{error::StorageError, parameters::StorageParameters};

#[async_trait]
pub trait Storage: Send + Sync {
    /// Save a small file that does not require streaming and can fit in-memory.
    fn save(
        &self,
        file: &Path,
        content: &[u8],
        parameters: StorageParameters,
    ) -> Result<(), StorageError>;

    /// Load a small file that does not require streaming and can fit in-memory.
    fn load(&self, file: &Path) -> Result<Vec<u8>, StorageError>;

    // Delete a stored file.
    fn delete(&self, file: &Path) -> Result<(), StorageError>;
}
