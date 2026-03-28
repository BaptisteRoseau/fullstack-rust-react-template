use storage::backends::S3;
use storage::testing::containers::{MINIO, TEST_BUCKET};

fn make_storage() -> S3 {
    S3::try_new(
        &MINIO.endpoint,
        TEST_BUCKET,
        &MINIO.access_key,
        &MINIO.secret_key,
    )
    .expect("failed to create S3 client")
}

storage::storage_trait_tests!(make_storage);

#[test]
fn test_minio_connection() {
    let _storage = make_storage();
}
