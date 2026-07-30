use std::path::{Path, PathBuf};

use uuid::Uuid;

use compressor::parameters::CompressionParameters;
use storage::Storage;
use test_utils::{trait_test, trait_test_suite};

/// Integration tests for the Storage trait, run against every backend.
///
/// When adding a test here:
/// - mark it `#[trait_test]` and take the subject as `&impl Storage`; the function
///   name becomes the test name, and that is the only place it is written
/// - helpers are regular functions, left alone by the macro
#[trait_test_suite]
pub mod suite {
    use super::*;

    #[trait_test]
    async fn save_and_load_compressed(storage: &impl Storage) {
        save_and_load_idempotent(storage, with_compression(), &unique_path()).await;
    }

    #[trait_test]
    async fn save_and_load(storage: &impl Storage) {
        save_and_load_idempotent(storage, no_compression(), &unique_path()).await;
    }

    #[trait_test]
    async fn save_overwrite(storage: &impl Storage) {
        let path = unique_path();
        let params = no_compression();

        storage
            .save(&path, b"version-1", &params)
            .await
            .expect("first save failed");
        storage
            .save(&path, b"version-2", &params)
            .await
            .expect("second save failed");

        let loaded = storage.load(&path).await.expect("load failed");
        assert_eq!(loaded, b"version-2");

        let _ = storage.delete(&path).await;
    }

    #[trait_test]
    async fn load_nonexistent(storage: &impl Storage) {
        let result = storage.load(&unique_path()).await;
        assert!(result.is_err(), "loading a nonexistent file should fail");
    }

    #[trait_test]
    async fn delete_nonexistent(storage: &impl Storage) {
        let result = storage.delete(&unique_path()).await;
        assert!(
            result.is_ok(),
            "deleting a nonexistent file should not result in an error"
        );
    }

    #[trait_test]
    async fn delete(storage: &impl Storage) {
        let path = unique_path();
        let params = no_compression();

        storage
            .save(&path, b"to be deleted", &params)
            .await
            .expect("save failed");
        storage.delete(&path).await.expect("delete failed");

        let result = storage.load(&path).await;
        assert!(result.is_err(), "load after delete should fail");
    }
}

/// Generate a unique test path to avoid blob collisions between parallel tests.
fn unique_path() -> PathBuf {
    PathBuf::from(format!("test-trait/{}", Uuid::new_v4()))
}

fn no_compression() -> CompressionParameters {
    CompressionParameters::default()
}

fn with_compression() -> CompressionParameters {
    *CompressionParameters::default().with_compression()
}

async fn save_and_load_idempotent(
    storage: &impl Storage,
    params: CompressionParameters,
    path: &Path,
) {
    let data = b"hello, storage!";

    storage
        .save(path, data, &params)
        .await
        .expect("save failed");
    let loaded = storage.load(path).await.expect("load failed");
    assert_eq!(loaded, data);

    let _ = storage.delete(path).await;
}
