use std::collections::HashMap;

use async_trait::async_trait;
use serde::{Serialize, de::DeserializeOwned};

use crate::error::CacheError;

#[async_trait]
pub trait Cache: Send + Sync {
    async fn set<T: Serialize + Send + Sync>(
        &self,
        key: &str,
        value: &T,
        timeout_s: Option<u32>,
    ) -> Result<(), CacheError>;

    async fn get<T: DeserializeOwned + Send>(&self, key: &str) -> Result<Option<T>, CacheError>;
    async fn delete(&self, key: &str) -> Result<(), CacheError>;

    async fn set_many<T: Serialize + Send + Sync>(
        &self,
        mappings: &HashMap<String, T>,
        timeout_s: Option<u32>,
    ) -> Result<(), CacheError>;

    async fn get_many<T: DeserializeOwned + Send>(
        &self,
        keys: &[&str],
    ) -> Result<HashMap<String, T>, CacheError>;

    async fn delete_many(&self, keys: &[&str]) -> Result<(), CacheError>;
}
