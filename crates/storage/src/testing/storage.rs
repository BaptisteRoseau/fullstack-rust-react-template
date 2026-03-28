use std::path::Path;

use crate::Storage;
use crate::parameters::StorageParameters;

// TODO: Test suite with different parameters.
// Parameterized tests + make a macro to avoid writing 5000 tests in the backends

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
