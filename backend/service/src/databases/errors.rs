use std::error::Error;
use thiserror::Error;
use tokio_postgres::error::SqlState;

#[derive(Error, Debug)]
pub(crate) enum DatabaseError {
    #[error("'{0}' does not exist")]
    NotFound(String),
    #[error("Unexpected length of the SQL column. Expected {expected:?}, got {got:?}")]
    InvalidColumn { expected: usize, got: usize },
    #[error("{0}")]
    AlreadyExists(String),
    #[error(transparent)]
    PostgresError(tokio_postgres::Error),
    #[error(transparent)]
    PoolBuildError(#[from] deadpool_postgres::BuildError),
    #[error(transparent)]
    PoolConfigError(#[from] deadpool_postgres::ConfigError),
    #[error(transparent)]
    PoolCreateError(#[from] deadpool_postgres::CreatePoolError),
    #[error(transparent)]
    PoolError(#[from] deadpool_postgres::PoolError),
    #[error(transparent)]
    PoolHookError(#[from] deadpool_postgres::HookError),
}

/*  Example of a Postgres error:
tokio_postgres::Error {
    kind: Db,
    cause: Some(DbError { severity: "ERROR",
    parsed_severity: Some(Error),
    code: SqlState(E23505),
    message: "duplicate key value violates unique constraint \"users_username_key\"",
    detail: Some("Key (username)=(test) already exists."),
    hint: None,
    position: None,
    where_: None,
    schema: Some("public"),
    table: Some("users"),
    column: None,
    datatype: None,
    constraint: Some("users_username_key"),
    file: Some("nbtinsert.c"),
    line: Some(666),
    routine: Some("_bt_check_unique") })
}

tokio_postgres::Error { kind: RowCount, cause: None }
*/

impl From<tokio_postgres::Error> for DatabaseError {
    // For error codes see:
    // - https://docs.rs/tokio-postgres/latest/tokio_postgres/error/struct.SqlState.html
    // - https://www.postgresql.org/docs/13/errcodes-appendix.html
    fn from(err: tokio_postgres::Error) -> Self {
        if let Some(code) = err.code() {
            return match *code {
                SqlState::UNIQUE_VIOLATION => {
                    // Detail example: "Key (username)=(test) already exists."
                    let detail = err
                        .source()
                        .and_then(|e| e.downcast_ref::<tokio_postgres::Error>())
                        .and_then(|e| e.source())
                        .and_then(|e| e.downcast_ref::<tokio_postgres::error::DbError>())
                        .and_then(|e| e.detail())
                        .map(|s| s.to_string())
                        .map(|s| s.replace("Key (username)=", ""))
                        .map(|s| s.replace('(', ""))
                        .map(|s| s.replace(')', ""))
                        .unwrap_or_else(|| err.to_string());
                    DatabaseError::AlreadyExists(detail)
                }
                SqlState::NO_DATA_FOUND => {
                    DatabaseError::NotFound(err.to_string().replace("ERROR: ", ""))
                }
                _ => DatabaseError::PostgresError(err),
            };
        }
        if format!("{:?}", err) == "Error { kind: RowCount, cause: None }" {
            return DatabaseError::NotFound("".to_string());
        }
        DatabaseError::PostgresError(err)
    }
}
