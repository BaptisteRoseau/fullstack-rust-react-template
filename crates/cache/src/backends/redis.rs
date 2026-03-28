use std::collections::HashMap;

use async_trait::async_trait;
use config::Config;
use deadpool_redis::{Pool, Runtime, redis::cmd};
use serde_json::Value;

use crate::{Cache, error::CacheError};

pub struct Redis {
    pool: Pool,
    timeout_s: Option<u32>,
}

impl Redis {
    pub fn new(url: &str, timeout_s: Option<u32>) -> Result<Self, CacheError> {
        let cfg = deadpool_redis::Config::from_url(url);
        let pool = cfg.create_pool(Some(Runtime::Tokio1))?;
        Ok(Self { pool, timeout_s })
    }
}

impl TryFrom<&Config> for Redis {
    type Error = CacheError;

    fn try_from(value: &Config) -> Result<Self, Self::Error> {
        Self::new(&value.redis.url, None)
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
        let serialized = serde_json::to_string(value)?;
        let mut conn = self.pool.get().await?;
        let ttl = timeout_s.or(self.timeout_s);
        match ttl {
            Some(seconds) => {
                cmd("SET")
                    .arg(key)
                    .arg(&serialized)
                    .arg("EX")
                    .arg(seconds)
                    .query_async::<()>(&mut conn)
                    .await?;
            }
            None => {
                cmd("SET")
                    .arg(key)
                    .arg(&serialized)
                    .query_async::<()>(&mut conn)
                    .await?;
            }
        }
        Ok(())
    }

    async fn get(&self, key: &str) -> Result<Option<Value>, CacheError> {
        let mut conn = self.pool.get().await?;
        let value: Option<String> = cmd("GET").arg(key).query_async(&mut conn).await?;
        match value {
            Some(serialized) => Ok(Some(serde_json::from_str(&serialized)?)),
            None => Ok(None),
        }
    }

    async fn delete(&self, key: &str) -> Result<(), CacheError> {
        let mut conn = self.pool.get().await?;
        cmd("DEL").arg(key).query_async::<()>(&mut conn).await?;
        Ok(())
    }

    async fn set_many(
        &self,
        mappings: &HashMap<String, Value>,
        timeout_s: Option<u32>,
    ) -> Result<(), CacheError> {
        for (key, value) in mappings {
            self.set(key, value, timeout_s).await?;
        }
        Ok(())
    }

    async fn get_many(&self, keys: &[&str]) -> Result<HashMap<String, Value>, CacheError> {
        let mut result = HashMap::new();
        for key in keys {
            if let Some(value) = self.get(key).await? {
                result.insert(key.to_string(), value);
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
            command.arg(*key);
        }
        command.query_async::<()>(&mut conn).await?;
        Ok(())
    }
}
