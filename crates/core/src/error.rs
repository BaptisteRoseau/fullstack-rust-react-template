use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error(transparent)]
    DatabaseError(#[from] database::error::DatabaseError),
}
