use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error(transparent)]
    ImageHandlingError(#[from] caesium::error::CaesiumError),
}
