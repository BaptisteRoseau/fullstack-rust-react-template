use std::collections::HashMap;

use async_trait::async_trait;
use config::Config;
use deadpool_redis::{Pool, Runtime, redis::cmd};
use serde_json::Value;
use tokio::task::JoinSet;

use crate::{Cache, error::CacheError};

pub struct Redis {
    pool: Pool,
    timeout_s: Option<u32>,
    prefix: Option<String>,
}

impl Redis {
    pub fn new(
        url: &str,
        timeout_s: Option<u32>,
        prefix: Option<String>,
    ) -> Result<Self, CacheError> {
        let cfg = deadpool_redis::Config::from_url(url);
        let pool = cfg.create_pool(Some(Runtime::Tokio1))?;
        Ok(Self {
            pool,
            timeout_s,
            prefix,
        })
    }

    fn prefixed_key(&self, key: &str) -> String {
        match &self.prefix {
            Some(prefix) => format!("{prefix}:{key}"),
            None => key.to_string(),
        }
    }
}

impl TryFrom<&Config> for Redis {
    type Error = CacheError;

    fn try_from(value: &Config) -> Result<Self, Self::Error> {
        Self::new(&value.redis.url, None, None)
    }
}

#[async_trait]
impl Cache for Redis {
    async fn set(
        &self,
        key: &str,
        value: &Value,
        timeout_s: Option<u32>,
    ) -> Result<(), CacheError> {
        let redis_key = self.prefixed_key(key);
        let serialized = serde_json::to_string(value)?;
        let mut conn = self.pool.get().await?;
        let ttl = timeout_s.or(self.timeout_s);
        match ttl {
            Some(seconds) => {
                cmd("SET")
                    .arg(&redis_key)
                    .arg(&serialized)
                    .arg("EX")
                    .arg(seconds)
                    .query_async::<()>(&mut conn)
                    .await?;
            }
            None => {
                cmd("SET")
                    .arg(&redis_key)
                    .arg(&serialized)
                    .query_async::<()>(&mut conn)
                    .await?;
            }
        }
        Ok(())
    }

    async fn get(&self, key: &str) -> Result<Option<Value>, CacheError> {
        let redis_key = self.prefixed_key(key);
        let mut conn = self.pool.get().await?;
        let value: Option<String> = cmd("GET").arg(&redis_key).query_async(&mut conn).await?;
        match value {
            Some(serialized) => Ok(Some(serde_json::from_str(&serialized)?)),
            None => Ok(None),
        }
    }

    async fn delete(&self, key: &str) -> Result<(), CacheError> {
        let redis_key = self.prefixed_key(key);
        let mut conn = self.pool.get().await?;
        cmd("DEL")
            .arg(&redis_key)
            .query_async::<()>(&mut conn)
            .await?;
        Ok(())
    }

    async fn set_many(
        &self,
        mappings: &HashMap<String, Value>,
        timeout_s: Option<u32>,
    ) -> Result<(), CacheError> {
        let mut tasks = JoinSet::new();
        for (key, value) in mappings {
            let pool = self.pool.clone();
            let redis_key = self.prefixed_key(key);
            let serialized = serde_json::to_string(value)?;
            let ttl = timeout_s.or(self.timeout_s);
            tasks.spawn(async move {
                let mut conn = pool.get().await?;
                match ttl {
                    Some(seconds) => {
                        cmd("SET")
                            .arg(&redis_key)
                            .arg(&serialized)
                            .arg("EX")
                            .arg(seconds)
                            .query_async::<()>(&mut conn)
                            .await?;
                    }
                    None => {
                        cmd("SET")
                            .arg(&redis_key)
                            .arg(&serialized)
                            .query_async::<()>(&mut conn)
                            .await?;
                    }
                }
                Ok::<(), CacheError>(())
            });
        }
        while let Some(result) = tasks.join_next().await {
            result.expect("set_many task panicked")?;
        }
        Ok(())
    }

    async fn get_many(
        &self,
        keys: &[&str],
    ) -> Result<HashMap<String, Value>, CacheError> {
        let mut tasks = JoinSet::new();
        for key in keys {
            let pool = self.pool.clone();
            let original_key = key.to_string();
            let redis_key = self.prefixed_key(key);
            tasks.spawn(async move {
                let mut conn = pool.get().await?;
                let value: Option<String> =
                    cmd("GET").arg(&redis_key).query_async(&mut conn).await?;
                let parsed = match value {
                    Some(serialized) => Some(serde_json::from_str(&serialized)?),
                    None => None,
                };
                Ok::<(String, Option<Value>), CacheError>((original_key, parsed))
            });
        }
        let mut result = HashMap::new();
        while let Some(join_result) = tasks.join_next().await {
            let (key, value) = join_result.expect("get_many task panicked")?;
            if let Some(v) = value {
                result.insert(key, v);
            }
        }
        Ok(result)
    }

    async fn delete_many(&self, keys: &[&str]) -> Result<(), CacheError> {
        if keys.is_empty() {
            return Ok(());
        }
        let mut conn = self.pool.get().await?;
        let mut command = cmd("DEL");
        for key in keys {
            command.arg(self.prefixed_key(key));
        }
        command.query_async::<()>(&mut conn).await?;
        Ok(())
    }
}
