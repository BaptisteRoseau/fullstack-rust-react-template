#[derive(thiserror::Error, Debug)]
pub enum CoreError {
    #[error(transparent)]
    DatabaseError(#[from] Box<database::error::DatabaseError>),
    #[error("Could not find {0}")]
    NotFound(String),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error(transparent)]
    AuthenticatorError(#[from] Box<authenticator::error::AuthenticatorError>),
}

impl CoreError {
    /// Keeps a missing row a `NotFound`, any other database failure stays a `DatabaseError`.
    pub(crate) fn from_database(error: database::error::DatabaseError) -> Self {
        match error {
            database::error::DatabaseError::NotFound(missing) => Self::NotFound(missing),
            other => Self::DatabaseError(Box::new(other)),
        }
    }
}

impl From<Box<database::error::DatabaseError>> for Box<CoreError> {
    fn from(value: Box<database::error::DatabaseError>) -> Self {
        Box::new(CoreError::DatabaseError(value))
    }
}
