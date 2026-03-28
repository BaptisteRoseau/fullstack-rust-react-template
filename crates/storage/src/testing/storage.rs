use std::path::Path;

use crate::Storage;
use crate::parameters::StorageParameters;

// When adding a new test here:
// - helpers are regular private functions
// - tests signature is `pub fn assert_<my test>(storage: &dyn Storage)`
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

        #[test]
        fn test_save_and_load() {
            assert_save_and_load(&$builder());
        }

        #[test]
        fn test_save_overwrite() {
            assert_save_overwrite(&$builder());
        }

        #[test]
        fn test_load_nonexistent() {
            assert_load_nonexistent(&$builder());
        }

        #[test]
        fn test_delete() {
            assert_delete(&$builder());
        }
    };
}

fn no_compression() -> StorageParameters {
    StorageParameters::default()
}
pub fn assert_save_and_load(storage: &dyn Storage) {
    let path = Path::new("test-trait/save_and_load.bin");
    let data = b"hello, storage!";
    let params = no_compression();

    storage.save(path, data, params).expect("save failed");
    let loaded = storage.load(path).expect("load failed");
    assert_eq!(loaded, data);

    let _ = storage.delete(path);
}

pub fn assert_save_overwrite(storage: &dyn Storage) {
    let path = Path::new("test-trait/save_overwrite.bin");
    let params = no_compression();

    storage
        .save(path, b"version-1", params)
        .expect("first save failed");
    storage
        .save(path, b"version-2", params)
        .expect("second save failed");

    let loaded = storage.load(path).expect("load failed");
    assert_eq!(loaded, b"version-2");

    let _ = storage.delete(path);
}

pub fn assert_load_nonexistent(storage: &dyn Storage) {
    let path = Path::new("test-trait/nonexistent.bin");
    let result = storage.load(path);
    assert!(result.is_err(), "loading a nonexistent file should fail");
}

pub fn assert_delete(storage: &dyn Storage) {
    let path = Path::new("test-trait/delete.bin");
    let params = no_compression();

    storage
        .save(path, b"to be deleted", params)
        .expect("save failed");
    storage.delete(path).expect("delete failed");

    let result = storage.load(path);
    assert!(result.is_err(), "load after delete should fail");
}
