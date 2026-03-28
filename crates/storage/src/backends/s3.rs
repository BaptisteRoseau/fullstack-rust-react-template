use config::Config;
use s3::{AddressingStyle, Auth, Client, Credentials};
use std::path::Path;

use crate::{
    Storage,
    compressor::compress_bytes,
    error::StorageError,
    images::compress_image,
    parameters::{Compression, StorageParameters},
};

// TODO: Save the parameters alongside the file

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
    ) -> Result<Self, StorageError> {
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
}

impl TryFrom<&Config> for S3 {
    type Error = StorageError;

    fn try_from(value: &Config) -> Result<Self, Self::Error> {
        Self::try_new(
            &value.s3.host,
            "default",
            &value.s3.user,
            &value.s3.password,
        )
    }
}

impl Storage for S3 {
    async fn save(
        &self,
        file: &Path,
        content: &[u8],
        parameters: StorageParameters,
    ) -> Result<(), StorageError> {
        let processed = match parameters.image {
            Some(image_compression_parameters) => {
                compress_image(content, &image_compression_parameters)?
            }
            None => content.into(),
        };

        let body = match parameters.compression {
            Compression::Gzip => compress_bytes(&processed)?,
            Compression::NoCompression => processed,
        };

        let key = Self::key_from_path(file);
        self.client
            .objects()
            .put(&self.bucket, &key)
            .body_bytes(body)
            .send()
            .await?;

        Ok(())
    }

    async fn load(&self, file: &Path) -> Result<Vec<u8>, StorageError> {
        let key = Self::key_from_path(file);
        let output = self.client.objects().get(&self.bucket, &key).send().await?;
        let raw = output.bytes().await?;
        let data = raw.to_vec();
        Ok(data)
    }

    async fn delete(&self, file: &Path) -> Result<(), StorageError> {
        let key = Self::key_from_path(file);
        self.client
            .objects()
            .delete(&self.bucket, &key)
            .send()
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::S3;
    use crate::testing::containers::MINIO;

    fn make_storage() -> S3 {
        S3::try_new(
            &MINIO.endpoint,
            crate::testing::containers::TEST_BUCKET,
            &MINIO.access_key,
            &MINIO.secret_key,
        )
        .expect("failed to create S3 client")
    }

    crate::storage_trait_tests!(make_storage);

    #[test]
    fn test_minio_connection() {
        let _storage = make_storage();
    }
}
