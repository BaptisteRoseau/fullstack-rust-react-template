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
        _file: &Path,
        _content: &[u8],
        _parameters: StorageParameters,
    ) -> Result<(), StorageError> {
        todo!()
    }

    fn load(
        &self,
        _file: &Path,
        _parameters: StorageParameters,
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
        _writer: &mut dyn Write,
        _parameters: StorageParameters,
    ) -> Result<(), StorageError> {
        todo!()
    }
}
