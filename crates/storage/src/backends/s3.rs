use config::Config;
use s3::{Auth, Client, Credentials};
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
    pub fn new(
        endpoint: &str,
        access_key: &str,
        secret_key: &str,
    ) -> Result<Self, StorageError> {
        let credentials = Credentials::new(access_key, secret_key)?;
        let client = Client::builder(endpoint)?
            .region("us-east-1")
            .auth(Auth::Static(credentials))
            .build()?;
        Ok(Self { client })
    }
}

impl TryFrom<&Config> for S3 {
    type Error = StorageError;

    fn try_from(value: &Config) -> Result<Self, Self::Error> {
        let credentials = Credentials::new(&value.s3.user, &value.s3.password)?;
        let client = Client::builder(&value.s3.host)?
            .region("us-east-1")
            .auth(Auth::Static(credentials))
            .build()?;
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

    fn delete(&self, _file: &Path) -> Result<(), StorageError> {
        todo!()
    }
}

#[cfg(test)]
#[cfg(feature = "integration")]
mod tests {
    use super::S3;
    use crate::testing::containers::minio::MINIO;

    fn make_storage() -> S3 {
        S3::new(&MINIO.endpoint, &MINIO.access_key, &MINIO.secret_key)
            .expect("failed to create S3 client")
    }

    #[test]
    fn test_minio_connection() {
        let _storage = make_storage();
    }

    // Uncomment as S3 trait methods get implemented:
    //
    // #[test]
    // fn test_save_and_load() {
    //     crate::testing::trait_tests::assert_save_and_load(&make_storage());
    // }
    //
    // #[test]
    // fn test_save_overwrite() {
    //     crate::testing::trait_tests::assert_save_overwrite(&make_storage());
    // }
    //
    // #[test]
    // fn test_load_nonexistent() {
    //     crate::testing::trait_tests::assert_load_nonexistent(&make_storage());
    // }
    //
    // #[test]
    // fn test_save_stream_and_load_stream() {
    //     crate::testing::trait_tests::assert_save_stream_and_load_stream(&make_storage());
    // }
    //
    // #[test]
    // fn test_delete() {
    //     crate::testing::trait_tests::assert_delete(&make_storage());
    // }
    //
    // #[test]
    // fn test_direct_save() {
    //     crate::testing::trait_tests::assert_direct_save(&make_storage());
    // }
    //
    // #[test]
    // fn test_direct_load() {
    //     crate::testing::trait_tests::assert_direct_load(&make_storage());
    // }
}
