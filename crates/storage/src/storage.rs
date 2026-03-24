use async_trait::async_trait;
use std::{
    io::{Read, Write},
    path::Path,
};

use crate::{error::StorageError, parameters::StorageParameters};

#[async_trait]
pub trait Storage: Send + Sync {
    /// Save a small file that does not require streaming and
    /// can fit in-memory.
    fn save(
        &self,
        file: &Path,
        content: &[u8],
        parameters: StorageParameters,
    ) -> Result<(), StorageError>;

    /// Load a small file that does not require streaming and
    /// can fit in-memory.
    fn load(
        &self,
        file: &Path,
        parameters: StorageParameters,
    ) -> Result<Vec<u8>, StorageError>;

    /// Save large files that cannot fit in-memory using streaming.
    fn save_stream(
        &self,
        reader: &mut dyn Read,
        parameters: StorageParameters,
    ) -> Result<(), StorageError>;

    /// Load large files that cannot fit in-memory using streaming.
    fn load_stream(
        &self,
        writer: &mut dyn Write,
        parameters: StorageParameters,
    ) -> Result<(), StorageError>;

    // Delete a stored file.
    fn delete(&self, file: &Path) -> Result<(), StorageError>;
}
