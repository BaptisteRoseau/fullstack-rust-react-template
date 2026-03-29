use async_trait::async_trait;
use sqlx::postgres::PgPoolOptions;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::crud::{CrudError, CrudExecutor, CrudValue};
#[warn(dead_code)]
use crate::database::Database;
use crate::error::DatabaseError;
use crate::models::{User, UserPatch};
use config::Config;
use tracing::warn;

#[derive(Clone)]
pub struct Postgres {
    pool: PgPool,
}

impl Postgres {
    pub async fn try_from(config: &Config) -> Result<Self, DatabaseError> {
        let url = format!(
            "postgres://{}:{}@{}:{}/{}",
            config.postgres.user,
            config.postgres.password,
            config.postgres.host,
            config.postgres.port,
            config.postgres.database,
        );
        let pool = PgPoolOptions::new().max_connections(10).connect(&url).await;

        match pool {
            Ok(pool) => Ok(Self { pool }),
            Err(e) => {
                warn!("Could not connect to database yet: {e}");
                // Create pool without connecting (lazy)
                let pool = PgPoolOptions::new()
                    .max_connections(10)
                    .connect_lazy(&url)?;
                Ok(Self { pool })
            }
        }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

fn bind_crud_value_query_as<'q, T>(
    query: sqlx::query::QueryAs<'q, sqlx::Postgres, T, sqlx::postgres::PgArguments>,
    value: CrudValue,
) -> sqlx::query::QueryAs<'q, sqlx::Postgres, T, sqlx::postgres::PgArguments>
where
    T: for<'r> FromRow<'r, sqlx::postgres::PgRow>,
{
    match value {
        CrudValue::Uuid(v) => query.bind(v),
        CrudValue::String(v) => query.bind(v),
        CrudValue::OptionString(v) => query.bind(v),
        CrudValue::DateTime(v) => query.bind(v),
        CrudValue::OptionDateTime(v) => query.bind(v),
        CrudValue::Bool(v) => query.bind(v),
        CrudValue::OptionBool(v) => query.bind(v),
        CrudValue::I32(v) => query.bind(v),
        CrudValue::OptionI32(v) => query.bind(v),
        CrudValue::I64(v) => query.bind(v),
        CrudValue::OptionI64(v) => query.bind(v),
        CrudValue::F64(v) => query.bind(v),
        CrudValue::OptionF64(v) => query.bind(v),
    }
}

fn bind_crud_value_query(
    query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
    value: CrudValue,
) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
    match value {
        CrudValue::Uuid(v) => query.bind(v),
        CrudValue::String(v) => query.bind(v),
        CrudValue::OptionString(v) => query.bind(v),
        CrudValue::DateTime(v) => query.bind(v),
        CrudValue::OptionDateTime(v) => query.bind(v),
        CrudValue::Bool(v) => query.bind(v),
        CrudValue::OptionBool(v) => query.bind(v),
        CrudValue::I32(v) => query.bind(v),
        CrudValue::OptionI32(v) => query.bind(v),
        CrudValue::I64(v) => query.bind(v),
        CrudValue::OptionI64(v) => query.bind(v),
        CrudValue::F64(v) => query.bind(v),
        CrudValue::OptionF64(v) => query.bind(v),
    }
}

#[async_trait]
impl CrudExecutor for Postgres {
    async fn crud_fetch_one<T>(
        &self,
        query: &str,
        args: Vec<CrudValue>,
    ) -> Result<T, CrudError>
    where
        T: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    {
        let mut q = sqlx::query_as::<_, T>(query);
        for arg in args {
            q = bind_crud_value_query_as(q, arg);
        }
        Ok(q.fetch_one(&self.pool).await?)
    }

    async fn crud_fetch_all<T>(
        &self,
        query: &str,
        args: Vec<CrudValue>,
    ) -> Result<Vec<T>, CrudError>
    where
        T: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    {
        let mut q = sqlx::query_as::<_, T>(query);
        for arg in args {
            q = bind_crud_value_query_as(q, arg);
        }
        Ok(q.fetch_all(&self.pool).await?)
    }

    async fn crud_execute(
        &self,
        query: &str,
        args: Vec<CrudValue>,
    ) -> Result<u64, CrudError> {
        let mut q = sqlx::query(query);
        for arg in args {
            q = bind_crud_value_query(q, arg);
        }
        Ok(q.execute(&self.pool).await?.rows_affected())
    }
}

#[async_trait]
impl Database for Postgres {
    async fn create_user(&mut self, _patch: UserPatch) -> Result<User, Box<DatabaseError>>{
        todo!()
    }
    async fn update_user(&mut self, _patch: UserPatch) -> Result<User, Box<DatabaseError>>{
        todo!()
    }
    async fn read_user(&self, uuid: Uuid) -> Result<User, Box<DatabaseError>>{
        todo!()
    }
    async fn delete_user(&mut self, uuid: Uuid) -> Result<(), Box<DatabaseError>>{
        todo!()
    }
}
