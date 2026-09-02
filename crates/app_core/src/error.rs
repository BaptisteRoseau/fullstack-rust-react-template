#[derive(thiserror::Error, Debug)]
pub enum CoreError {
    #[error(transparent)]
    DatabaseError(#[from] Box<database::error::DatabaseError>),
    #[error("Could not find {0}")]
    NotFound(String),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    /// The caller asked for something the domain refuses: an unknown permission
    /// level, a directory moved inside itself, a share granted to oneself.
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    #[error(transparent)]
    StorageError(#[from] Box<storage::error::StorageError>),
    #[error(transparent)]
    CompressorError(#[from] Box<compressor::error::CompressorError>),
    #[error("Compression error: {0}")]
    Compression(#[from] std::io::Error),
    /// An AES-GCM operation failed. The message stays deliberately vague: a
    /// caller must not learn whether it was the key, the nonce or the tag.
    #[error("Encryption error")]
    Encryption,
}

impl From<Box<database::error::DatabaseError>> for Box<CoreError> {
    fn from(value: Box<database::error::DatabaseError>) -> Self {
        Box::new(CoreError::DatabaseError(value))
    }
}
