use std::{
    io::{Read, Write},
    path::Path,
};

use crate::{error::StorageError, parameters::StorageParameters};

// This interface is subject to change as I implement
// backends support.
pub trait Storage {
    fn save(
        &self,
        file: Path,
        content: &[u8],
        parameters: StorageParameters,
    ) -> Result<(), StorageError>;
    fn load<W: Write>(
        &self,
        file: Path,
        parameters: StorageParameters,
    ) -> Result<Vec<u8>, StorageError>;
    fn save_stream<R: Read>(
        &self,
        reader: R,
        parameters: StorageParameters,
    ) -> Result<(), StorageError>;
    fn load_stream<W: Write>(
        &self,
        writer: W,
        parameters: StorageParameters,
    ) -> Result<(), StorageError>;
}
