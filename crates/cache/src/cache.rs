use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use async_trait::async_trait;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::error::CacheError;

/// Compute a deterministic cache key by JSON-serializing, then hashing.
pub fn cache_key<K: Serialize + ?Sized>(key: &K) -> Result<String, CacheError> {
    let serialized = serde_json::to_string(key)?;
    Ok(hash_string(&serialized))
}

fn hash_string(s: &str) -> String {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

/// Cache trait with a dyn-safe backend interface and generic convenience methods.
///
/// Backends implement the `*_raw` methods, which operate on string keys and
/// `serde_json::Value` values. These methods are dyn-compatible and used by
/// code holding `dyn Cache` (e.g. `AppState`).
///
/// The generic convenience methods (`set`, `get`, `delete`, …) are available
/// through `impl Cache` only (`where Self: Sized`). They serialize keys and
/// values to JSON, hash the key, and delegate to the raw methods.
#[async_trait]
pub trait Cache: Send + Sync {
    // ── Backend interface (dyn-safe, implemented by backends) ────────────

    async fn set_raw(
        &self,
        key: &str,
        value: &Value,
        timeout_s: Option<u32>,
    ) -> Result<(), CacheError>;

    async fn get_raw(&self, key: &str) -> Result<Option<Value>, CacheError>;
    async fn delete_raw(&self, key: &str) -> Result<(), CacheError>;

    async fn set_many_raw(
        &self,
        mappings: &HashMap<String, Value>,
        timeout_s: Option<u32>,
    ) -> Result<(), CacheError>;

    async fn get_many_raw(
        &self,
        keys: &[&str],
    ) -> Result<HashMap<String, Value>, CacheError>;

    async fn delete_many_raw(&self, keys: &[&str]) -> Result<(), CacheError>;

    // ── Generic convenience methods (not dyn-safe) ──────────────────────

    async fn set<K: Serialize + Send + Sync, V: Serialize + Send + Sync>(
        &self,
        key: &K,
        value: &V,
        timeout_s: Option<u32>,
    ) -> Result<(), CacheError>
    where
        Self: Sized,
    {
        let hashed = cache_key(key)?;
        let json_value = serde_json::to_value(value)?;
        self.set_raw(&hashed, &json_value, timeout_s).await
    }

    async fn get<K: Serialize + Send + Sync, V: DeserializeOwned + Send>(
        &self,
        key: &K,
    ) -> Result<Option<V>, CacheError>
    where
        Self: Sized,
    {
        let hashed = cache_key(key)?;
        match self.get_raw(&hashed).await? {
            Some(v) => Ok(Some(serde_json::from_value(v)?)),
            None => Ok(None),
        }
    }

    async fn delete<K: Serialize + Send + Sync>(
        &self,
        key: &K,
    ) -> Result<(), CacheError>
    where
        Self: Sized,
    {
        let hashed = cache_key(key)?;
        self.delete_raw(&hashed).await
    }

    async fn set_many<V: Serialize + Send + Sync>(
        &self,
        mappings: &HashMap<String, V>,
        timeout_s: Option<u32>,
    ) -> Result<(), CacheError>
    where
        Self: Sized,
    {
        let raw_mappings: HashMap<String, Value> = mappings
            .iter()
            .map(|(k, v)| Ok((cache_key(k)?, serde_json::to_value(v)?)))
            .collect::<Result<_, CacheError>>()?;
        self.set_many_raw(&raw_mappings, timeout_s).await
    }

    async fn get_many<V: DeserializeOwned + Send>(
        &self,
        keys: &[&str],
    ) -> Result<HashMap<String, V>, CacheError>
    where
        Self: Sized,
    {
        let pairs: Vec<(String, &str)> =
            keys.iter().map(|k| (hash_string(&serde_json::to_string(k).expect("string serialization cannot fail")), *k)).collect();
        let hashed: Vec<&str> = pairs.iter().map(|(h, _)| h.as_str()).collect();
        let raw = self.get_many_raw(&hashed).await?;
        let mut result = HashMap::new();
        for (hash, original) in &pairs {
            if let Some(v) = raw.get(hash.as_str()) {
                result.insert(
                    original.to_string(),
                    serde_json::from_value(v.clone())?,
                );
            }
        }
        Ok(result)
    }

    async fn delete_many(&self, keys: &[&str]) -> Result<(), CacheError>
    where
        Self: Sized,
    {
        let hashed: Vec<String> = keys
            .iter()
            .map(|k| hash_string(&serde_json::to_string(k).expect("string serialization cannot fail")))
            .collect();
        let hashed_refs: Vec<&str> = hashed.iter().map(|s| s.as_str()).collect();
        self.delete_many_raw(&hashed_refs).await
    }
}
