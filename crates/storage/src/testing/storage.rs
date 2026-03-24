use std::io::Cursor;
use std::path::Path;

use crate::Storage;
use crate::parameters::StorageParameters;

/// Returns parameters suitable for testing with raw bytes (no image/gzip processing).
fn raw_params() -> StorageParameters {
    let mut params = StorageParameters::new();
    params.no_compression();
    params.no_image_compression();
    params
}

pub fn assert_save_and_load(storage: &dyn Storage) {
    let path = Path::new("test-trait/save_and_load.bin");
    let data = b"hello, storage!";
    let params = raw_params();

    storage.save(path, data, params).expect("save failed");
    let loaded = storage.load(path, params).expect("load failed");
    assert_eq!(loaded, data);

    let _ = storage.delete(path);
}

pub fn assert_save_overwrite(storage: &dyn Storage) {
    let path = Path::new("test-trait/save_overwrite.bin");
    let params = raw_params();

    storage
        .save(path, b"version-1", params)
        .expect("first save failed");
    storage
        .save(path, b"version-2", params)
        .expect("second save failed");

    let loaded = storage.load(path, params).expect("load failed");
    assert_eq!(loaded, b"version-2");

    let _ = storage.delete(path);
}

pub fn assert_load_nonexistent(storage: &dyn Storage) {
    let path = Path::new("test-trait/nonexistent.bin");
    let params = raw_params();

    let result = storage.load(path, params);
    assert!(result.is_err(), "loading a nonexistent file should fail");
}

pub fn assert_save_stream_and_load_stream(storage: &dyn Storage) {
    let data = b"streamed content for testing";
    let mut reader = Cursor::new(data);
    let params = raw_params();

    storage
        .save_stream(&mut reader, params)
        .expect("save_stream failed");

    let mut output = Vec::new();
    storage
        .load_stream(&mut output, params)
        .expect("load_stream failed");
    assert_eq!(output, data);
}

pub fn assert_delete(storage: &dyn Storage) {
    let path = Path::new("test-trait/delete.bin");
    let params = raw_params();

    storage
        .save(path, b"to be deleted", params)
        .expect("save failed");
    storage.delete(path).expect("delete failed");

    let result = storage.load(path, params);
    assert!(result.is_err(), "load after delete should fail");
}
