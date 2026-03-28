use std::path::Path;

use crate::{error::StorageError, parameters::StorageParameters};

pub trait Storage: Send + Sync {
    /// Save a small file that does not require streaming and can fit in-memory.
    fn save(
        &self,
        file: &Path,
        content: &[u8],
        parameters: StorageParameters,
    ) -> impl Future<Output = Result<(), Box<StorageError>>> + Send;

    /// Load a small file that does not require streaming and can fit in-memory.
    fn load(&self, file: &Path) -> impl Future<Output = Result<Vec<u8>, Box<StorageError>>> + Send;

    // Delete a stored file.
    fn delete(&self, file: &Path) -> impl Future<Output = Result<(), Box<StorageError>>> + Send;
}
