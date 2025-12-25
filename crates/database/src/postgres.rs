use super::database::Database;
use super::errors::DatabaseError;
use super::models::StoredLoginInfo;
use crate::config::Config;
use deadpool_postgres::{Config as DpConfig, ManagerConfig, Pool, RecyclingMethod, Runtime};
use models::{
    company::{Company, CompanyContent},
    user::{User, UserContent},
};
use tokio_postgres::types::ToSql;
use tokio_postgres::{NoTls, Row};
use tracing::warn;

// TODO: Efficiently manage to cache size
// TODO: Require SSL when enabled in config & when using release config
#[derive(Clone)]
pub(crate) struct PostgresDatabase {
    pool: Pool,
}

impl PostgresDatabase {
    pub(crate) async fn from(config: &Config) -> Result<Self, DatabaseError> {
        let cfg = Self::parameters(config)?;
        let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;
        if pool.get().await.is_err() {
            warn!("Could not connect to database yet");
        }
        Ok(Self { pool })
    }

    fn parameters(config: &Config) -> Result<DpConfig, DatabaseError> {
        let mut dp_config = DpConfig::new();
        dp_config.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Clean,
        });
        dp_config.user = Some(config.postgres.user.clone());
        dp_config.host = Some(config.postgres.host.clone());
        dp_config.dbname = Some(config.postgres.database.clone());
        dp_config.password = Some(config.postgres.password.clone());
        dp_config.port = Some(config.postgres.port);
        Ok(dp_config)
    }

    pub async fn query_one_cached<T: ToString>(
        &self,
        query: T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Row, DatabaseError> {
        let client = self.pool.get().await?;
        let statement = client.prepare_cached(query.to_string().as_str()).await?;
        let row = client.query_one(&statement, params).await?;
        Ok(row)
    }

    pub async fn query_one<T: ToString>(
        &self,
        query: T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Row, DatabaseError> {
        let client = self.pool.get().await?;
        let row = client.query_one(query.to_string().as_str(), params).await?;
        Ok(row)
    }

    pub async fn execute_cached<T: ToString>(
        &self,
        query: T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<u64, DatabaseError> {
        let client = self.pool.get().await?;
        let statement = client.prepare_cached(query.to_string().as_str()).await?;
        Ok(client.execute(&statement, params).await?)
    }

    pub async fn execute<T: ToString>(
        &self,
        query: T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<u64, DatabaseError> {
        let client = self.pool.get().await?;
        let affected = client.execute(query.to_string().as_str(), params).await?;
        Ok(affected)
    }
}

impl Database for PostgresDatabase {
    async fn close(&mut self) -> Result<(), DatabaseError> {
        self.pool.close();
        Ok(())
    }

    async fn init(&mut self, config: &Config) -> Result<&mut Self, DatabaseError> {
        let _ = config;
        Ok(self)
    }
}

// TESTS: See https://testcontainers.com/ & https://docs.rs/testcontainers/latest/testcontainers/
//TODO: Rename tests modules as unit_tests and integration_tests
// to be able to launch one or the other rapidly
