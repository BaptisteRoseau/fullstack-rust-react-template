use std::path::PathBuf;

use uuid::Uuid;

use storage::Storage;
use storage::parameters::StorageParameters;

// When adding a new test here:
// - helpers are regular private functions
// - tests signature is `pub async fn assert_<my test>(storage: &impl Storage)`
// - new tests should be added in the test file that uses them (e.g. s3.rs)

/// Generate a unique test path to avoid blob collisions between parallel tests.
fn unique_path(name: &str) -> PathBuf {
    PathBuf::from(format!("test-trait/{}/{name}", Uuid::new_v4()))
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
    path: &PathBuf,
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
    let path = unique_path("save_and_load_compressed.bin");
    save_and_load_idempotent(storage, with_compression(), &path).await;
}

pub async fn assert_save_and_load(storage: &impl Storage) {
    let path = unique_path("save_and_load.bin");
    save_and_load_idempotent(storage, no_compression(), &path).await;
}

pub async fn assert_save_overwrite(storage: &impl Storage) {
    let path = unique_path("save_overwrite.bin");
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
    let path = unique_path("nonexistent.bin");
    let result = storage.load(&path).await;
    assert!(result.is_err(), "loading a nonexistent file should fail");
}

pub async fn assert_delete_nonexistent(storage: &impl Storage) {
    let path = unique_path("nonexistent.bin");
    let result = storage.delete(&path).await;
    assert!(
        result.is_ok(),
        "deleting a nonexistent file should not result in an error"
    );
}

pub async fn assert_delete(storage: &impl Storage) {
    let path = unique_path("delete.bin");
    let params = no_compression();

    storage
        .save(&path, b"to be deleted", &params)
        .await
        .expect("save failed");
    storage.delete(&path).await.expect("delete failed");

    let result = storage.load(&path).await;
    assert!(result.is_err(), "load after delete should fail");
}
