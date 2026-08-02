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
