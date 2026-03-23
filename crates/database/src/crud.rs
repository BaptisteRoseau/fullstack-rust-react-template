use async_trait::async_trait;
use sqlx::FromRow;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CrudError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error("Not found: {0}")]
    NotFound(String),
}

/// Type-erased query parameter value.
///
/// Used by the Crud derive macro to pass parameters to the CrudExecutor
/// without coupling generated code to a specific database driver.
#[derive(Debug, Clone)]
pub enum CrudValue {
    Uuid(uuid::Uuid),
    String(String),
    OptionString(Option<String>),
    DateTime(chrono::DateTime<chrono::Utc>),
    OptionDateTime(Option<chrono::DateTime<chrono::Utc>>),
    Bool(bool),
    OptionBool(Option<bool>),
    I32(i32),
    OptionI32(Option<i32>),
    I64(i64),
    OptionI64(Option<i64>),
    F64(f64),
    OptionF64(Option<f64>),
}

/// Abstraction for executing CRUD queries.
///
/// Implemented by database backends (e.g. Postgres) to decouple
/// the generated CRUD methods from a specific connection pool type.
#[async_trait]
pub trait CrudExecutor: Send + Sync {
    async fn crud_fetch_one<T>(&self, query: &str, args: Vec<CrudValue>) -> Result<T, CrudError>
    where
        T: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin;

    async fn crud_fetch_all<T>(&self, query: &str, args: Vec<CrudValue>) -> Result<Vec<T>, CrudError>
    where
        T: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin;

    async fn crud_execute(&self, query: &str, args: Vec<CrudValue>) -> Result<u64, CrudError>;
}
