use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error(transparent)]
    S3Error(#[from] s3::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    CompressionError(#[from] compressor::error::CompressorError),
}

impl From<Box<compressor::error::CompressorError>> for Box<StorageError> {
    fn from(e: Box<compressor::error::CompressorError>) -> Self {
        Box::new(StorageError::CompressionError(*e))
    }
}

impl From<s3::Error> for Box<StorageError> {
    fn from(e: s3::Error) -> Self {
        Box::new(StorageError::S3Error(e))
    }
}

impl From<std::io::Error> for Box<StorageError> {
    fn from(e: std::io::Error) -> Self {
        Box::new(StorageError::IoError(e))
    }
}
