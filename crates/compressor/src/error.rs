use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompressorError {
    #[error(transparent)]
    ImageHandlingError(#[from] caesium::error::CaesiumError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

impl From<caesium::error::CaesiumError> for Box<CompressorError> {
    fn from(e: caesium::error::CaesiumError) -> Self {
        Box::new(CompressorError::ImageHandlingError(e))
    }
}

impl From<std::io::Error> for Box<CompressorError> {
    fn from(e: std::io::Error) -> Self {
        Box::new(CompressorError::IoError(e))
    }
}
