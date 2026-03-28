use std::path::{Path, PathBuf};

use uuid::Uuid;

use storage::Storage;
use storage::parameters::StorageParameters;

// When adding a new test here:
// - helpers are regular private functions
// - tests signature is `pub async fn assert_<my test>(storage: &impl Storage)`
// - new tests should be added in the `storage_trait_tests` macro

/// Set of integration tests for the Storage trait.
/// Simply pass it a function to create the object that implements the trait.
/// The caller must have `mod common;` declared beforehand.
///
/// For example:
///
/// ```rs
/// mod common;
///
/// use common::containers::{MINIO, TEST_BUCKET};
/// use storage::backends::S3;
///
/// fn make_storage() -> S3 { /* ... */ }
///
/// storage_trait_tests!(make_storage);
/// ```
macro_rules! storage_trait_tests {
    ($builder:expr) => {
        use common::storage::*;

        #[tokio::test]
        async fn test_save_and_load_compressed() {
            assert_save_and_load_compressed(&$builder()).await;
        }

        #[tokio::test]
        async fn test_save_and_load() {
            assert_save_and_load(&$builder()).await;
        }

        #[tokio::test]
        async fn test_save_overwrite() {
            assert_save_overwrite(&$builder()).await;
        }

        #[tokio::test]
        async fn test_load_nonexistent() {
            assert_load_nonexistent(&$builder()).await;
        }

        #[tokio::test]
        async fn test_delete_nonexistent() {
            assert_delete_nonexistent(&$builder()).await;
        }

        #[tokio::test]
        async fn test_delete() {
            assert_delete(&$builder()).await;
        }
    };
}

/// Generate a unique test path to avoid blob collisions between parallel tests.
fn unique_path() -> PathBuf {
    PathBuf::from(format!("test-trait/{}", Uuid::new_v4()))
}

fn no_compression() -> StorageParameters {
    StorageParameters::default()
}

fn with_compression() -> StorageParameters {
    *StorageParameters::default().with_compression()
}

async fn save_and_load_idempotent(
    storage: &impl Storage,
    params: StorageParameters,
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

pub async fn assert_save_and_load_compressed(storage: &impl Storage) {
    save_and_load_idempotent(storage, with_compression(), &unique_path()).await;
}

pub async fn assert_save_and_load(storage: &impl Storage) {
    save_and_load_idempotent(storage, no_compression(), &unique_path()).await;
}

pub async fn assert_save_overwrite(storage: &impl Storage) {
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

pub async fn assert_load_nonexistent(storage: &impl Storage) {
    let result = storage.load(&unique_path()).await;
    assert!(result.is_err(), "loading a nonexistent file should fail");
}

pub async fn assert_delete_nonexistent(storage: &impl Storage) {
    let result = storage.delete(&unique_path()).await;
    assert!(
        result.is_ok(),
        "deleting a nonexistent file should not result in an error"
    );
}

pub async fn assert_delete(storage: &impl Storage) {
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
