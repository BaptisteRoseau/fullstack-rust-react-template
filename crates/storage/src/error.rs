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
