use config::Config;
use s3::{AddressingStyle, Auth, Client, Credentials};
use std::path::Path;

use async_trait::async_trait;

use crate::{
    Storage,
    compressor::{handle_compression, handle_decompression},
    error::StorageError,
    images::compress_image,
    parameters::{Compression, StorageParameters},
};

#[derive(Clone)]
pub struct S3 {
    client: Client,
    bucket: String,
}

impl S3 {
    pub fn try_new(
        endpoint: &str,
        bucket: &str,
        access_key: &str,
        secret_key: &str,
    ) -> Result<Self, Box<StorageError>> {
        let credentials = Credentials::new(access_key, secret_key)?;
        let client = Client::builder(endpoint)?
            .region("myregion")
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

    fn content_type_for(compression: Compression) -> &'static str {
        match compression {
            Compression::Gzip => "application/gzip",
            Compression::NoCompression => "application/octet-stream",
        }
    }

    fn compression_from_content_type(content_type: &Option<String>) -> Compression {
        match content_type.as_deref() {
            Some("application/gzip") => Compression::Gzip,
            _ => Compression::NoCompression,
        }
    }
}

impl TryFrom<&Config> for S3 {
    type Error = Box<StorageError>;

    fn try_from(value: &Config) -> Result<Self, Self::Error> {
        Self::try_new(
            &value.s3.host,
            "default",
            &value.s3.user,
            &value.s3.password,
        )
    }
}

#[async_trait]
impl Storage for S3 {
    async fn save(
        &self,
        file: &Path,
        content: &[u8],
        parameters: &StorageParameters,
    ) -> Result<(), Box<StorageError>> {
        let processed = compress_image(content, &parameters.image)?;

        let body = handle_compression(&processed, parameters.compression)?;

        let key = Self::key_from_path(file);
        self.client
            .objects()
            .put(&self.bucket, &key)
            .body_bytes(body)
            .content_type(Self::content_type_for(parameters.compression))
            .send()
            .await?;

        Ok(())
    }

    async fn load(&self, file: &Path) -> Result<Vec<u8>, Box<StorageError>> {
        let key = Self::key_from_path(file);
        let output = self.client.objects().get(&self.bucket, &key).send().await?;
        let compression = Self::compression_from_content_type(&output.content_type);
        let raw = output.bytes().await?;
        let data = handle_decompression(&raw, compression)?;
        Ok(data)
    }

    async fn delete(&self, file: &Path) -> Result<(), Box<StorageError>> {
        let key = Self::key_from_path(file);
        self.client
            .objects()
            .delete(&self.bucket, &key)
            .send()
            .await?;
        Ok(())
    }
}
