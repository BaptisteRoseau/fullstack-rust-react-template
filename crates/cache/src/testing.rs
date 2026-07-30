//! In-memory [`Cache`] test double, shared by downstream crates' tests.
//!
//! Enable via the `test-utils` feature (add it under `[dev-dependencies]`, not
//! `[dependencies]`, so it never leaks into non-test builds).

use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use async_trait::async_trait;
use serde_json::Value;

use crate::cache::Cache;
use crate::error::CacheError;

/// In-memory [`Cache`] backed by a `HashMap`, ignoring expiry timeouts.
#[derive(Default)]
pub struct MockCache {
    store: Mutex<HashMap<String, Value>>,
}

impl MockCache {
    /// Locks the store, recovering from poisoning instead of propagating it:
    /// a poisoned lock in a test double should not abort the test run.
    fn store(&self) -> MutexGuard<'_, HashMap<String, Value>> {
        self.store.lock().unwrap_or_else(|e| e.into_inner())
    }
}

#[async_trait]
impl Cache for MockCache {
    async fn set(
        &self,
        key: &str,
        value: &Value,
        _timeout_s: Option<u32>,
    ) -> Result<(), CacheError> {
        self.store().insert(key.to_string(), value.clone());
        Ok(())
    }

    async fn get(&self, key: &str) -> Result<Option<Value>, CacheError> {
        Ok(self.store().get(key).cloned())
    }

    async fn delete(&self, key: &str) -> Result<(), CacheError> {
        self.store().remove(key);
        Ok(())
    }

    async fn set_many(
        &self,
        mappings: &HashMap<String, Value>,
        _timeout_s: Option<u32>,
    ) -> Result<(), CacheError> {
        let mut store = self.store();
        for (key, value) in mappings {
            store.insert(key.clone(), value.clone());
        }
        Ok(())
    }

    async fn get_many(
        &self,
        keys: &[&str],
    ) -> Result<HashMap<String, Value>, CacheError> {
        let store = self.store();
        Ok(keys
            .iter()
            .filter_map(|key| store.get(*key).map(|v| (key.to_string(), v.clone())))
            .collect())
    }

    async fn delete_many(&self, keys: &[&str]) -> Result<(), CacheError> {
        let mut store = self.store();
        for key in keys {
            store.remove(*key);
        }
        Ok(())
    }
}
