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

    async fn get_many(&self, keys: &[&str])
    -> Result<HashMap<String, Value>, CacheError>;
    async fn delete_many(&self, keys: &[&str]) -> Result<(), CacheError>;

    /** ==========================================================
     * NO FAIL variants
     * =========================================================== */
    async fn set_nofail(&self, key: &str, value: &Value, timeout_s: Option<u32>) {
        let _ = self.set(key, value, timeout_s).await;
    }

    async fn get_nofail(&self, key: &str) -> Option<Value> {
        self.get(key).await.unwrap_or(None)
    }

    async fn delete_nofail(&self, key: &str) {
        let _ = self.delete(key).await;
    }

    async fn set_many_nofail(
        &self,
        mappings: &HashMap<String, Value>,
        timeout_s: Option<u32>,
    ) {
        let _ = self.set_many(mappings, timeout_s).await;
    }

    async fn get_many_nofail(&self, keys: &[&str]) -> HashMap<String, Value> {
        self.get_many(keys).await.unwrap_or(HashMap::new())
    }

    async fn delete_many_nofail(&self, keys: &[&str]) {
        let _ = self.delete_many(keys).await;
    }
}
