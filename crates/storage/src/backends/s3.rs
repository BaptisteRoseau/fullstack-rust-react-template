use config::Config;
use s3::Client;
use std::{
    io::{Read, Write},
    path::Path,
};

use crate::{Storage, error::StorageError, parameters::StorageParameters};

#[derive(Clone)]
pub struct S3 {
    client: Client,
}

impl TryFrom<&Config> for S3 {
    type Error = StorageError;

    fn try_from(value: &Config) -> Result<Self, Self::Error> {
        let client = Client::builder(&value.s3.host)?.build()?;
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

    fn load(
        &self,
        file: &Path,
        parameters: StorageParameters,
    ) -> Result<Vec<u8>, StorageError> {
        todo!()
    }

    fn save_stream(
        &self,
        _reader: &mut dyn Read,
        _parameters: StorageParameters,
    ) -> Result<(), StorageError> {
        todo!()
    }

    fn load_stream(
        &self,
        writer: &mut dyn Write,
        parameters: StorageParameters,
    ) -> Result<(), StorageError> {
        todo!()
    }

    fn direct_save(&self, file: &Path) -> Result<(), StorageError> {
        todo!()
    }

    fn direct_load(&self, file: &Path) -> Result<Vec<u8>, StorageError> {
        todo!()
    }

    fn delete(&self, file: &Path) -> Result<(), StorageError> {
        todo!()
    }
}
