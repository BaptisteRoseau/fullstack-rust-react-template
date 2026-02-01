use thiserror::Error;

use crate::parameters::ImageConvertion;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error(transparent)]
    ImageHandlingError(#[from] caesium::error::CaesiumError),
}
