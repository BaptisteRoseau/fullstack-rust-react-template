//! In-process [`Cache`] backend backed by a `HashMap`.

use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};
use std::time::{Duration, Instant};

use async_trait::async_trait;
use serde_json::Value;

use crate::cache::Cache;
use crate::error::CacheError;

/// How much time must pass between two sweeps of expired entries.
///
/// Reads drop expired entries as they find them, but a key nobody reads again
/// would hold its value forever. Writes sweep the whole map to reclaim those,
/// throttled to this interval so a write-heavy caller does not pay O(n) each
/// time.
const SWEEP_INTERVAL: Duration = Duration::from_secs(60);

/// [`Cache`] held in this process' memory, backed by a `HashMap`.
///
/// Useful wherever a cache is wanted but a shared one is not: single-process
/// deployments, and any test that needs a working cache rather than a stub.
/// Nothing is shared between processes and nothing survives a restart.
///
/// Expiry is honoured, matching [`Redis`](crate::backends::redis::Redis): a
/// per-call `timeout_s` wins, otherwise the default passed to [`new`](Self::new)
/// applies, and `None` on both means the entry stays until deleted.
#[derive(Default)]
pub struct HashMapCache {
    store: Mutex<Store>,
    /// Applied when a call passes no `timeout_s` of its own.
    timeout_s: Option<u32>,
}

impl HashMapCache {
    /// Builds a cache whose entries expire after `timeout_s` unless the caller
    /// passes its own. `None` means entries stay until deleted.
    pub fn new(timeout_s: Option<u32>) -> Self {
        Self {
            store: Mutex::new(Store::default()),
            timeout_s,
        }
    }

    /// Locks the store, recovering from poisoning instead of propagating it: one
    /// panicking caller should not take the whole cache down with it.
    fn store(&self) -> MutexGuard<'_, Store> {
        self.store.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Resolves a call's `timeout_s` against the default into a deadline.
    ///
    /// A timeout far enough out to overflow the clock yields `None`, i.e. never
    /// expires — the nearest honest answer, and unreachable in practice since
    /// `u32` seconds tops out at ~136 years.
    fn deadline(&self, timeout_s: Option<u32>, now: Instant) -> Option<Instant> {
        timeout_s
            .or(self.timeout_s)
            .and_then(|seconds| now.checked_add(Duration::from_secs(u64::from(seconds))))
    }
}

/// An entry and the instant it stops being visible, if it has one.
struct Entry {
    value: Value,
    expires_at: Option<Instant>,
}

impl Entry {
    fn is_expired(&self, now: Instant) -> bool {
        self.expires_at.is_some_and(|deadline| deadline <= now)
    }
}

struct Store {
    entries: HashMap<String, Entry>,
    last_sweep: Instant,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            entries: HashMap::new(),
            last_sweep: Instant::now(),
        }
    }
}

impl Store {
    /// Reads `key`, treating an expired entry as absent and dropping it.
    fn get(&mut self, key: &str, now: Instant) -> Option<Value> {
        let entry = self.entries.get(key)?;
        if !entry.is_expired(now) {
            return Some(entry.value.clone());
        }
        self.entries.remove(key);
        None
    }

    fn insert(&mut self, key: String, value: Value, expires_at: Option<Instant>) {
        self.entries.insert(key, Entry { value, expires_at });
    }

    /// Drops every expired entry, at most once per [`SWEEP_INTERVAL`].
    fn sweep_if_due(&mut self, now: Instant) {
        if now.duration_since(self.last_sweep) < SWEEP_INTERVAL {
            return;
        }
        self.entries.retain(|_, entry| !entry.is_expired(now));
        self.last_sweep = now;
    }
}

#[async_trait]
impl Cache for HashMapCache {
    async fn set(
        &self,
        key: &str,
        value: &Value,
        timeout_s: Option<u32>,
    ) -> Result<(), CacheError> {
        let now = Instant::now();
        let expires_at = self.deadline(timeout_s, now);
        let mut store = self.store();
        store.sweep_if_due(now);
        store.insert(key.to_string(), value.clone(), expires_at);
        Ok(())
    }

    async fn get(&self, key: &str) -> Result<Option<Value>, CacheError> {
        Ok(self.store().get(key, Instant::now()))
    }

    async fn delete(&self, key: &str) -> Result<(), CacheError> {
        self.store().entries.remove(key);
        Ok(())
    }

    async fn set_many(
        &self,
        mappings: &HashMap<String, Value>,
        timeout_s: Option<u32>,
    ) -> Result<(), CacheError> {
        let now = Instant::now();
        let expires_at = self.deadline(timeout_s, now);
        let mut store = self.store();
        store.sweep_if_due(now);
        for (key, value) in mappings {
            store.insert(key.clone(), value.clone(), expires_at);
        }
        Ok(())
    }

    async fn get_many(
        &self,
        keys: &[&str],
    ) -> Result<HashMap<String, Value>, CacheError> {
        let now = Instant::now();
        let mut store = self.store();
        Ok(keys
            .iter()
            .filter_map(|key| store.get(key, now).map(|value| (key.to_string(), value)))
            .collect())
    }

    async fn delete_many(&self, keys: &[&str]) -> Result<(), CacheError> {
        let mut store = self.store();
        for key in keys {
            store.entries.remove(*key);
        }
        Ok(())
    }
}

/* =======================================================================================
* TESTS
======================================================================================= */

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    /// The sweep is throttled to [`SWEEP_INTERVAL`], so a fresh cache never runs
    /// one and expired-but-unread entries stay in the map. Backdating
    /// `last_sweep` is the only way to observe the reclaim without sleeping a
    /// minute; the trait suite covers the visible behaviour.
    #[tokio::test]
    async fn sweep_reclaims_expired_entries_nobody_reads() {
        let cache = HashMapCache::default();
        cache
            .set("stale", &json!("v"), Some(0))
            .await
            .expect("set failed");

        let retained = cache.store().entries.len();
        assert_eq!(
            retained, 1,
            "entry should still be in the map, got {retained}"
        );

        cache.store().last_sweep = Instant::now() - SWEEP_INTERVAL;
        cache
            .set("fresh", &json!("v"), None)
            .await
            .expect("set failed");

        let keys: Vec<String> = cache.store().entries.keys().cloned().collect();
        assert_eq!(
            keys,
            vec!["fresh".to_string()],
            "expected only the unexpired entry to survive the sweep, got {keys:?}"
        );
    }

    /// `timeout_s` on the call wins over the cache-wide default, the same way
    /// `Redis` resolves it.
    #[tokio::test]
    async fn per_call_timeout_overrides_the_default() {
        let cache = HashMapCache::new(Some(0));
        cache
            .set("key", &json!("v"), Some(3600))
            .await
            .expect("set failed");

        let value = cache.get("key").await.expect("get failed");
        assert_eq!(
            value,
            Some(json!("v")),
            "the per-call timeout should have kept the entry alive, got {value:?}"
        );
    }

    #[tokio::test]
    async fn default_timeout_applies_when_the_call_passes_none() {
        let cache = HashMapCache::new(Some(0));
        cache
            .set("key", &json!("v"), None)
            .await
            .expect("set failed");

        let value = cache.get("key").await.expect("get failed");
        assert!(
            value.is_none(),
            "the default timeout should have expired the entry, got {value:?}"
        );
    }
}
