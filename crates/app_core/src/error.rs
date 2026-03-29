#[derive(thiserror::Error, Debug)]
pub enum CoreError {
    #[error(transparent)]
    DatabaseError(#[from] Box<database::error::DatabaseError>),
    #[error("Could not find {0}")]
    NotFound(String),
}
