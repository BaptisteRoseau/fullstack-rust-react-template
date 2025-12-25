use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error(transparent)]
    Postgres(#[from] tokio_postgres::error::Error),
    #[error(transparent)]
    ConnectionPool(#[from] deadpool_postgres::PoolError),
    #[error(transparent)]
    PoolCreationError(#[from] deadpool_postgres::CreatePoolError),
}
