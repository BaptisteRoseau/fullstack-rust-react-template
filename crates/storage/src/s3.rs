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

impl S3 {
    pub fn try_new(config: &Config) -> Result<Self, StorageError> {
        let client = Client::builder(&config.s3.host)?.build()?;
        Ok(Self { client })
    }
}
