use std::path::{Path, PathBuf};

use uuid::Uuid;

use storage::Storage;
use storage::parameters::StorageParameters;

// When adding a new test here:
// - helpers are regular private functions
// - tests signature is `pub async fn assert_<my test>(storage: &impl Storage)`
// - new tests should be added in the `storage_trait_tests` macro

/// Set of integration tests for the Storage trait.
/// Returns a `Vec<Trial>` for use with `libtest-mimic`.
/// The caller must have `mod common;` declared beforehand.
///
/// For example:
///
/// ```rs
/// mod common;
///
/// use common::containers::{MinioFixture, TEST_BUCKET};
/// use storage::backends::S3;
///
/// fn main() {
///     let rt = tokio::runtime::Runtime::new().unwrap();
///     let fixture = rt.block_on(async { /* ... */ });
///     let make_storage = || S3::try_new(/* ... */).unwrap();
///     let tests = storage_trait_tests!(make_storage, &rt);
///     let conclusion = libtest_mimic::run(&args, tests);
///     drop(fixture);
///     conclusion.exit();
/// }
/// ```
macro_rules! storage_trait_tests {
    ($builder:expr, $rt:expr) => {{
        use common::storage::*;
        use libtest_mimic::Trial;
        use std::sync::Arc;

        let rt: Arc<tokio::runtime::Runtime> = $rt;
        let builder = Arc::new($builder);

        vec![
            {
                let rt = rt.clone();
                let builder = builder.clone();
                Trial::test("save_and_load_compressed", move || {
                    rt.block_on(assert_save_and_load_compressed(&builder()));
                    Ok(())
                })
            },
            {
                let rt = rt.clone();
                let builder = builder.clone();
                Trial::test("save_and_load", move || {
                    rt.block_on(assert_save_and_load(&builder()));
                    Ok(())
                })
            },
            {
                let rt = rt.clone();
                let builder = builder.clone();
                Trial::test("save_overwrite", move || {
                    rt.block_on(assert_save_overwrite(&builder()));
                    Ok(())
                })
            },
            {
                let rt = rt.clone();
                let builder = builder.clone();
                Trial::test("load_nonexistent", move || {
                    rt.block_on(assert_load_nonexistent(&builder()));
                    Ok(())
                })
            },
            {
                let rt = rt.clone();
                let builder = builder.clone();
                Trial::test("delete_nonexistent", move || {
                    rt.block_on(assert_delete_nonexistent(&builder()));
                    Ok(())
                })
            },
            {
                let rt = rt.clone();
                let builder = builder.clone();
                Trial::test("delete", move || {
                    rt.block_on(assert_delete(&builder()));
                    Ok(())
                })
            },
        ]
    }};
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
