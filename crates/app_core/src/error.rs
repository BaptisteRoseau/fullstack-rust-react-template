#[derive(thiserror::Error, Debug)]
pub enum CoreError {
    #[error(transparent)]
    DatabaseError(#[from] Box<database::error::DatabaseError>),
    #[error("Could not find {0}")]
    NotFound(String),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

impl From<Box<database::error::DatabaseError>> for Box<CoreError> {
    fn from(value: Box<database::error::DatabaseError>) -> Self {
        Box::new(CoreError::DatabaseError(value))
    }
}
