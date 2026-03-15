use async_trait::async_trait;
use std::{
    io::{Read, Write},
    path::Path,
};

use crate::{error::StorageError, parameters::StorageParameters};

#[async_trait]
pub trait Storage: Send + Sync {
    fn save(
        &self,
        file: &Path,
        content: &[u8],
        parameters: StorageParameters,
    ) -> Result<(), StorageError>;

    fn load(
        &self,
        file: &Path,
        parameters: StorageParameters,
    ) -> Result<Vec<u8>, StorageError>;

    fn save_stream(
        &self,
        reader: &mut dyn Read,
        parameters: StorageParameters,
    ) -> Result<(), StorageError>;

    fn load_stream(
        &self,
        writer: &mut dyn Write,
        parameters: StorageParameters,
    ) -> Result<(), StorageError>;
}
