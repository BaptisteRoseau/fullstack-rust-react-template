use std::collections::HashMap;

use async_trait::async_trait;
use serde_json::Value;

use crate::error::CacheError;

/// A dyn-compatible cache trait.
///
/// Values are stored as `serde_json::Value` so the trait remains object-safe
/// and can be used as `dyn Cache`. Callers serialize/deserialize on their side
/// using `serde_json::to_value` / `serde_json::from_value`.
#[async_trait]
pub trait Cache: Send + Sync {
    async fn set(
        &self,
        key: &str,
        value: &Value,
        timeout_s: Option<u32>,
    ) -> Result<(), CacheError>;

    async fn get(&self, key: &str) -> Result<Option<Value>, CacheError>;
    async fn delete(&self, key: &str) -> Result<(), CacheError>;

    async fn set_many(
        &self,
        mappings: &HashMap<String, Value>,
        timeout_s: Option<u32>,
    ) -> Result<(), CacheError>;

    async fn get_many(&self, keys: &[&str]) -> Result<HashMap<String, Value>, CacheError>;
    async fn delete_many(&self, keys: &[&str]) -> Result<(), CacheError>;
}
