use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error(transparent)]
    Postgres(#[from] tokio_postgres::error::Error),
}
