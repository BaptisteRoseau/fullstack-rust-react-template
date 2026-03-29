use thiserror::Error;

use crate::crud::CrudError;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error(transparent)]
    Crud(#[from] CrudError),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error("The item was not found.")]
    NotFound(String),
}

impl From<CrudError> for Box<DatabaseError> {
    fn from(value: CrudError) -> Self {
        Box::new(DatabaseError::Crud(value))
    }
}

impl From<sqlx::Error> for Box<DatabaseError> {
    fn from(value: sqlx::Error) -> Self {
        Box::new(DatabaseError::Sqlx(value))
    }
}
