mod common;

use common::containers::{MINIO, TEST_BUCKET};
use common::storage::*;
use storage::backends::S3;

fn make_storage() -> S3 {
    S3::try_new(
        &MINIO.endpoint,
        TEST_BUCKET,
        &MINIO.access_key,
        &MINIO.secret_key,
    )
    .expect("failed to create S3 client")
}

#[test]
fn test_minio_connection() {
    let _storage = make_storage();
}

#[tokio::test]
async fn test_save_and_load_compressed() {
    assert_save_and_load_compressed(&make_storage()).await;
}

#[tokio::test]
async fn test_save_and_load() {
    assert_save_and_load(&make_storage()).await;
}

#[tokio::test]
async fn test_save_overwrite() {
    assert_save_overwrite(&make_storage()).await;
}

#[tokio::test]
async fn test_load_nonexistent() {
    assert_load_nonexistent(&make_storage()).await;
}

#[tokio::test]
async fn test_delete_nonexistent() {
    assert_delete_nonexistent(&make_storage()).await;
}

#[tokio::test]
async fn test_delete() {
    assert_delete(&make_storage()).await;
}
