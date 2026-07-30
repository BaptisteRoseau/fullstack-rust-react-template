use std::path::Path;

use async_trait::async_trait;

use crate::error::StorageError;
use compressor::parameters::CompressionParameters;

#[async_trait]
pub trait Storage: Send + Sync {
    /// Save a small file that does not require streaming and can fit in-memory.
    async fn save(
        &self,
        file: &Path,
        content: &[u8],
        parameters: &CompressionParameters,
    ) -> Result<(), Box<StorageError>>;

    /// Load a small file that does not require streaming and can fit in-memory.
    async fn load(&self, file: &Path) -> Result<Vec<u8>, Box<StorageError>>;

    // Delete a stored file.
    async fn delete(&self, file: &Path) -> Result<(), Box<StorageError>>;
}
