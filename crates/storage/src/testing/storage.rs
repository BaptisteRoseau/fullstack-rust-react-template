use std::path::Path;

use crate::Storage;
use crate::parameters::StorageParameters;

// When adding a new test here:
// - helpers are regular private functions
// - tests signature is `pub async fn assert_<my test>(storage: &impl Storage)`
// - new tests should be added in the `storage_trait_tests` macro

/// Set of unit tests for the storage trait.
/// Simply pass it a function to create the object that implements the trait.
///
/// For example:
///
/// ```rs
// fn make_storage() -> S3 {
//     S3::try_new(
//         &MINIO.endpoint,
//         crate::testing::containers::TEST_BUCKET,
//         &MINIO.access_key,
//         &MINIO.secret_key,
//     )
//     .expect("failed to create S3 client")
// }
//
// storage_trait_tests!(make_storage)
/// ```
#[macro_export]
macro_rules! storage_trait_tests {
    ($builder:expr) => {
        use $crate::testing::storage::*;

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

fn no_compression() -> StorageParameters {
    StorageParameters::default()
}

fn with_compression() -> StorageParameters {
    *StorageParameters::default().with_compression()
}

async fn save_and_load_idempotent(storage: &impl Storage, params: StorageParameters) {
    let path = Path::new("test-trait/save_and_load.bin");
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
    save_and_load_idempotent(storage, with_compression()).await;
}

pub async fn assert_save_and_load(storage: &impl Storage) {
    save_and_load_idempotent(storage, no_compression()).await;
}

pub async fn assert_save_overwrite(storage: &impl Storage) {
    let path = Path::new("test-trait/save_overwrite.bin");
    let params = no_compression();

    storage
        .save(path, b"version-1", &params)
        .await
        .expect("first save failed");
    storage
        .save(path, b"version-2", &params)
        .await
        .expect("second save failed");

    let loaded = storage.load(path).await.expect("load failed");
    assert_eq!(loaded, b"version-2");

    let _ = storage.delete(path).await;
}

pub async fn assert_load_nonexistent(storage: &impl Storage) {
    let path = Path::new("test-trait/nonexistent.bin");
    let result = storage.load(path).await;
    assert!(result.is_err(), "loading a nonexistent file should fail");
}

pub async fn assert_delete_nonexistent(storage: &impl Storage) {
    let path = Path::new("test-trait/nonexistent.bin");
    let result = storage.delete(path).await;
    assert!(result.is_err(), "deleting a nonexistent file should fail");
}

pub async fn assert_delete(storage: &impl Storage) {
    let path = Path::new("test-trait/delete.bin");
    let params = no_compression();

    storage
        .save(path, b"to be deleted", &params)
        .await
        .expect("save failed");
    storage.delete(path).await.expect("delete failed");

    let result = storage.load(path).await;
    assert!(result.is_err(), "load after delete should fail");
}
