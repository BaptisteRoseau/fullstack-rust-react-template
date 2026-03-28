use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error(transparent)]
    ImageHandlingError(#[from] caesium::error::CaesiumError),
    #[error(transparent)]
    S3Error(#[from] s3::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

impl From<caesium::error::CaesiumError> for Box<StorageError> {
    fn from(e: caesium::error::CaesiumError) -> Self {
        Box::new(StorageError::ImageHandlingError(e))
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
