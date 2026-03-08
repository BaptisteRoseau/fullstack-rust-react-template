use config::Config;
use s3::{Auth, Client};
use std::{
    io::{Read, Write},
    path::Path,
};

use crate::{Storage, error::StorageError, parameters::StorageParameters};

pub struct S3 {
    client: Client,
}

impl S3 {
    pub fn try_new(config: &Config) -> Result<Self, StorageError> {
        let client = Client::builder(&config.s3.host)?.build()?;
        Ok(Self { client })
    }
}

impl Storage for S3 {
    fn save(
        &self,
        file: &Path,
        content: &[u8],
        parameters: StorageParameters,
    ) -> Result<(), StorageError> {
        todo!()
    }
    fn load<W: Write>(
        &self,
        file: &Path,
        parameters: StorageParameters,
    ) -> Result<Vec<u8>, StorageError> {
        todo!()
    }
    fn save_stream<R: Read>(
        &self,
        reader: R,
        parameters: StorageParameters,
    ) -> Result<(), StorageError> {
        todo!()
    }
    fn load_stream<W: Write>(
        &self,
        writer: W,
        parameters: StorageParameters,
    ) -> Result<(), StorageError> {
        todo!()
    }
}
