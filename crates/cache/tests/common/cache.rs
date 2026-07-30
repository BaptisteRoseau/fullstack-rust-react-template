use std::collections::HashMap;
use std::time::Duration;

use cache::Cache;
// use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use test_trait::{test_trait, test_trait_suite};

/// Integration tests for the Cache trait, run against every backend.
///
/// When adding a test here:
/// - mark it `#[test_trait]` and take the subject as `&impl Cache`; the function
///   name becomes the test name, and that is the only place it is written
/// - helpers are regular functions, left alone by the macro
#[test_trait_suite]
pub mod suite {
    use super::*;

    #[test_trait]
    async fn set_and_get(cache: &impl Cache) {
        let key = unique_key("set_and_get");
        let input = json!("hello");
        cache.set(&key, &input, None).await.expect("set failed");
        let output = cache.get(&key).await.expect("get failed");
        assert_eq!(
            output,
            Some(input),
            "expected Some(\"hello\"), got {output:?}"
        );
        let _ = cache.delete(&key).await;
    }

    #[test_trait]
    async fn get_nonexistent(cache: &impl Cache) {
        let key = unique_key("nonexistent");
        let value = cache.get(&key).await.expect("get failed");
        assert!(
            value.is_none(),
            "expected None for nonexistent key, got {value:?}"
        );
    }

    #[test_trait]
    async fn overwrite(cache: &impl Cache) {
        let key = unique_key("overwrite");
        let input1 = json!("v1");
        let input2 = json!("v2");
        cache
            .set(&key, &input1, None)
            .await
            .expect("first set failed");
        cache
            .set(&key, &input2, None)
            .await
            .expect("second set failed");
        let value = cache.get(&key).await.expect("get failed");
        assert_eq!(
            value,
            Some(input2),
            "expected Some(\"v2\") after overwrite, got {value:?}"
        );
        let _ = cache.delete(&key).await;
    }

    #[test_trait]
    async fn delete(cache: &impl Cache) {
        let key = unique_key("delete");
        cache
            .set(&key, &json!("to_delete"), None)
            .await
            .expect("set failed");
        cache.delete(&key).await.expect("delete failed");
        let value = cache.get(&key).await.expect("get after delete failed");
        assert!(value.is_none(), "expected None after delete, got {value:?}");
    }

    #[test_trait]
    async fn delete_nonexistent(cache: &impl Cache) {
        let key = unique_key("delete_nonexistent");
        let result = cache.delete(&key).await;
        assert!(
            result.is_ok(),
            "deleting a nonexistent key should not throw error, got {result:?}"
        );
    }

    #[test_trait]
    async fn set_with_timeout(cache: &impl Cache) {
        let key = unique_key("timeout");
        let input = json!("ephemeral");
        cache
            .set(&key, &input, Some(10))
            .await
            .expect("set with timeout failed");
        let value = cache.get(&key).await.expect("get failed");
        assert_eq!(
            value,
            Some(input),
            "expected Some(\"ephemeral\") before expiry, got {value:?}"
        );
        let _ = cache.delete(&key).await;
    }

    /// The only trial that sleeps. `timeout_s` is in seconds, so there is no
    /// shorter way to watch an entry age out, and a backend that accepts the
    /// timeout without honouring it is otherwise indistinguishable from one that
    /// does.
    #[test_trait]
    async fn set_with_timeout_expires(cache: &impl Cache) {
        let key = unique_key("expiry");
        let input = json!("ephemeral");
        cache
            .set(&key, &input, Some(1))
            .await
            .expect("set with timeout failed");

        tokio::time::sleep(Duration::from_millis(1500)).await;

        let value = cache.get(&key).await.expect("get failed");
        assert!(
            value.is_none(),
            "expected None once the 1s timeout elapsed, got {value:?}"
        );
    }

    #[test_trait]
    async fn set_many_and_get_many(cache: &impl Cache) {
        let prefix = unique_key("many");
        let k1 = format!("{prefix}:a");
        let k2 = format!("{prefix}:b");
        let k3 = format!("{prefix}:c");

        let mut mappings: HashMap<String, Value> = HashMap::new();
        mappings.insert(k1.clone(), json!("alpha"));
        mappings.insert(k2.clone(), json!("beta"));
        mappings.insert(k3.clone(), json!("gamma"));

        cache
            .set_many(&mappings, None)
            .await
            .expect("set_many failed");

        let mut keys: Vec<&str> = mappings.keys().map(|k| k.as_str()).collect();
        let k4 = format!("{prefix}:d");
        keys.push(k4.as_str());
        let result = cache.get_many(&keys).await.expect("get_many failed");

        assert_eq!(result.len(), 3, "expected 3 results, got {}", result.len());
        assert_eq!(
            result.get(&k1),
            Some(&json!("alpha")),
            "k1 mismatch: {result:?}"
        );
        assert_eq!(
            result.get(&k2),
            Some(&json!("beta")),
            "k2 mismatch: {result:?}"
        );
        assert_eq!(
            result.get(&k3),
            Some(&json!("gamma")),
            "k3 mismatch: {result:?}"
        );

        let _ = cache.delete_many(&keys).await;
    }

    #[test_trait]
    async fn get_many_nonexistent(cache: &impl Cache) {
        let prefix = unique_key("many");
        let k1 = format!("{prefix}:a");
        let k2 = format!("{prefix}:b");
        let k3 = format!("{prefix}:c");
        let keys = vec![k1.as_str(), k2.as_str(), k3.as_str()];
        let result = cache.get_many(&keys).await.expect("get_many failed");
        assert_eq!(result.len(), 0);
    }

    #[test_trait]
    async fn delete_many(cache: &impl Cache) {
        let prefix = unique_key("delete_many");
        let k1 = format!("{prefix}:a");
        let k2 = format!("{prefix}:b");

        cache
            .set(&k1, &json!("one"), None)
            .await
            .expect("set k1 failed");
        cache
            .set(&k2, &json!("two"), None)
            .await
            .expect("set k2 failed");

        let k3 = format!("{prefix}:b");
        let keys: Vec<&str> = vec![k1.as_str(), k2.as_str(), k3.as_str()];
        cache.delete_many(&keys).await.expect("delete_many failed");

        let v1 = cache.get(&k1).await.expect("get k1 failed");
        let v2 = cache.get(&k2).await.expect("get k2 failed");
        let v3 = cache.get(&k3).await.expect("get k2 failed");
        assert!(
            v1.is_none(),
            "expected None for k1 after delete_many, got {v1:?}"
        );
        assert!(
            v2.is_none(),
            "expected None for k2 after delete_many, got {v2:?}"
        );
        assert!(
            v3.is_none(),
            "expected None for k2 after delete_many, got {v2:?}"
        );
    }

    // #[derive(Serialize, Deserialize)]
    // struct SerdeStruct {
    //     field1: String,
    //     field2: u32
    // }
    //
    // #[test_trait]
    // async fn set_and_get_serde_struct(cache: &impl Cache) {
    //     let ukey = unique_key("set_and_get");
    //     let key = SerdeStruct{ field1: ukey, field2: 42 };
    //     let value = SerdeStruct{ field1: "Hello".into(), field2: 69 };
    //
    //     cache.set(&key, &value, None).await.expect("set failed");
    //     let output = cache.get(&key).await.expect("get failed");
    //     assert_eq!(output, Some(value), "expected Some(\"hello\"), got {output:?}");
    //     let _ = cache.delete(&key).await;
    // }
}

/// Unique key prefix to avoid collisions between parallel tests.
fn unique_key(suffix: &str) -> String {
    format!("test:{}:{suffix}", uuid::Uuid::new_v4())
}
