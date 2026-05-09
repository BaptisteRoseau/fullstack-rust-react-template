use async_trait::async_trait;
use sqlx::postgres::PgPoolOptions;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::crud::{CrudError, CrudExecutor, CrudValue};
#[warn(dead_code)]
use crate::database::Database;
use crate::error::DatabaseError;
use crate::models::{ApiKey, User, UserPatch};
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

    pub async fn try_from_url(url: &str) -> Result<Self, DatabaseError> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(url)
            .await?;
        Ok(Self { pool })
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
        CrudValue::Json(v) => query.bind(v),
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
        CrudValue::Json(v) => query.bind(v),
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
    async fn create_user(
        &mut self,
        patch: UserPatch,
    ) -> Result<User, Box<DatabaseError>> {
        Ok(patch.execute(self).await?)
    }
    async fn update_user(
        &mut self,
        patch: UserPatch,
    ) -> Result<User, Box<DatabaseError>> {
        Ok(patch.execute(self).await?)
    }
    async fn read_user(&self, uuid: Uuid) -> Result<User, Box<DatabaseError>> {
        let q = sqlx::query_as::<_, User>("SELECT * FROM user where id == %s").bind(uuid);
        Ok(q.fetch_one(&self.pool).await?)
    }
    async fn delete_user(&mut self, uuid: Uuid) -> Result<bool, Box<DatabaseError>> {
        let q = sqlx::query("DELETE * FROM user where id == %s").bind(uuid);
        Ok(q.execute(&self.pool).await?.rows_affected() == 1)
    }

    async fn create_api_key(
        &mut self,
        owner: Uuid,
        name: String,
        hash: String,
        permissions: serde_json::Value,
    ) -> Result<ApiKey, Box<DatabaseError>> {
        let result = sqlx::query_as::<_, ApiKey>(
            "INSERT INTO api_key (owner, name, hash, permissions) VALUES ($1, $2, $3, $4) RETURNING *",
        )
        .bind(owner)
        .bind(name)
        .bind(hash)
        .bind(permissions)
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(key) => Ok(key),
            Err(sqlx::Error::Database(db_err))
                if db_err.constraint() == Some("api_key_hash_key") =>
            {
                Err(Box::new(DatabaseError::HashCollision))
            }
            Err(e) => Err(Box::new(DatabaseError::Sqlx(e))),
        }
    }

    async fn read_api_key_by_id(&self, id: Uuid) -> Result<ApiKey, Box<DatabaseError>> {
        sqlx::query_as::<_, ApiKey>("SELECT * FROM api_key WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => {
                    Box::new(DatabaseError::NotFound(id.to_string()))
                }
                other => Box::new(DatabaseError::Sqlx(other)),
            })
    }

    async fn read_api_key_by_hash(&self, hash: &str) -> Result<ApiKey, Box<DatabaseError>> {
        sqlx::query_as::<_, ApiKey>("SELECT * FROM api_key WHERE hash = $1")
            .bind(hash)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => {
                    Box::new(DatabaseError::NotFound("api_key".to_string()))
                }
                other => Box::new(DatabaseError::Sqlx(other)),
            })
    }

    async fn delete_api_key(&mut self, id: Uuid) -> Result<bool, Box<DatabaseError>> {
        let rows = sqlx::query("DELETE FROM api_key WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| Box::new(DatabaseError::Sqlx(e)))?;
        Ok(rows.rows_affected() == 1)
    }
}
