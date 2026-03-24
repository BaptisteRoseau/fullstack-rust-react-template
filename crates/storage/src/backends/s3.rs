use config::Config;
use s3::{AddressingStyle, Auth, BlockingClient, Credentials};
use std::{
    io::{Read, Write},
    path::Path,
};

use crate::{
    Storage,
    compressor::{compress_bytes, decompress_bytes},
    error::StorageError,
    images::compress_image,
    parameters::{Compression, StorageParameters},
};

#[derive(Clone)]
pub struct S3 {
    client: BlockingClient,
    bucket: String,
}

impl S3 {
    pub fn new(
        endpoint: &str,
        bucket: &str,
        access_key: &str,
        secret_key: &str,
    ) -> Result<Self, StorageError> {
        let credentials = Credentials::new(access_key, secret_key)?;
        let client = BlockingClient::builder(endpoint)?
            .region("us-east-1")
            .auth(Auth::Static(credentials))
            .addressing_style(AddressingStyle::Path)
            .build()?;
        Ok(Self {
            client,
            bucket: bucket.to_string(),
        })
    }

    fn key_from_path(file: &Path) -> String {
        file.to_string_lossy().to_string()
    }
}

impl TryFrom<&Config> for S3 {
    type Error = StorageError;

    fn try_from(value: &Config) -> Result<Self, Self::Error> {
        Self::new(
            &value.s3.host,
            "default",
            &value.s3.user,
            &value.s3.password,
        )
    }
}

impl Storage for S3 {
    fn save(
        &self,
        file: &Path,
        content: &[u8],
        parameters: StorageParameters,
    ) -> Result<(), StorageError> {
        let processed = compress_image(content, parameters.image())?;

        let body = match parameters.compression() {
            Compression::Gzip => compress_bytes(&processed)?,
            Compression::NoCompression => processed,
        };

        let key = Self::key_from_path(file);
        self.client
            .objects()
            .put(&self.bucket, &key)
            .body_bytes(body)
            .send()?;

        Ok(())
    }

    fn load(
        &self,
        file: &Path,
        parameters: StorageParameters,
    ) -> Result<Vec<u8>, StorageError> {
        let key = Self::key_from_path(file);
        let output = self
            .client
            .objects()
            .get(&self.bucket, &key)
            .send()?;

        let raw = output.bytes()?;

        let data = match parameters.compression() {
            Compression::Gzip => decompress_bytes(&raw)?,
            Compression::NoCompression => raw.to_vec(),
        };

        Ok(data)
    }

    fn save_stream(
        &self,
        _reader: &mut dyn Read,
        _parameters: StorageParameters,
    ) -> Result<(), StorageError> {
        todo!("save_stream requires a file path in the trait signature")
    }

    fn load_stream(
        &self,
        _writer: &mut dyn Write,
        _parameters: StorageParameters,
    ) -> Result<(), StorageError> {
        todo!("load_stream requires a file path in the trait signature")
    }

    fn direct_save(&self, _file: &Path) -> Result<(), StorageError> {
        todo!("direct_save needs content to upload or should return a presigned URL")
    }

    fn direct_load(&self, file: &Path) -> Result<Vec<u8>, StorageError> {
        let key = Self::key_from_path(file);
        let output = self
            .client
            .objects()
            .get(&self.bucket, &key)
            .send()?;
        let raw = output.bytes()?;
        Ok(raw.to_vec())
    }

    fn delete(&self, file: &Path) -> Result<(), StorageError> {
        let key = Self::key_from_path(file);
        self.client
            .objects()
            .delete(&self.bucket, &key)
            .send()?;
        Ok(())
    }
}

#[cfg(test)]
#[cfg(feature = "integration")]
mod tests {
    use super::S3;
    use crate::testing::containers::minio::MINIO;

    fn make_storage() -> S3 {
        S3::new(
            &MINIO.endpoint,
            crate::testing::containers::minio::TEST_BUCKET,
            &MINIO.access_key,
            &MINIO.secret_key,
        )
        .expect("failed to create S3 client")
    }

    #[test]
    fn test_minio_connection() {
        let _storage = make_storage();
    }

    #[test]
    fn test_save_and_load() {
        crate::testing::trait_tests::assert_save_and_load(&make_storage());
    }

    #[test]
    fn test_save_overwrite() {
        crate::testing::trait_tests::assert_save_overwrite(&make_storage());
    }

    #[test]
    fn test_load_nonexistent() {
        crate::testing::trait_tests::assert_load_nonexistent(&make_storage());
    }

    #[test]
    fn test_delete() {
        crate::testing::trait_tests::assert_delete(&make_storage());
    }

    #[test]
    fn test_direct_load() {
        crate::testing::trait_tests::assert_direct_load(&make_storage());
    }
}
