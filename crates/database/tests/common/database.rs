use database::backends::Postgres;
use database::Database;
use uuid::Uuid;

#[macro_export]
macro_rules! database_trait_tests {
    ($builder:expr, $rt:expr) => {{
        use common::database::*;
        use libtest_mimic::Trial;
        use std::sync::Arc;

        let rt: Arc<tokio::runtime::Runtime> = $rt;
        let builder = Arc::new($builder);

        vec![
            {
                let rt = rt.clone();
                let builder = builder.clone();
                Trial::test("create_api_key", move || {
                    rt.block_on(async { assert_create_api_key((builder)().await).await });
                    Ok(())
                })
            },
            {
                let rt = rt.clone();
                let builder = builder.clone();
                Trial::test("read_api_key_by_hash", move || {
                    rt.block_on(async {
                        assert_read_api_key_by_hash((builder)().await).await
                    });
                    Ok(())
                })
            },
            {
                let rt = rt.clone();
                let builder = builder.clone();
                Trial::test("delete_api_key", move || {
                    rt.block_on(async {
                        assert_delete_api_key((builder)().await).await
                    });
                    Ok(())
                })
            },
            {
                let rt = rt.clone();
                let builder = builder.clone();
                Trial::test("delete_api_key_nonexistent", move || {
                    rt.block_on(async {
                        assert_delete_api_key_nonexistent((builder)().await).await
                    });
                    Ok(())
                })
            },
        ]
    }};
}

/// Insert a minimal user row directly via sqlx and return its id.
/// Uses a unique email per call to avoid conflicts between parallel tests.
async fn create_test_user(db: &Postgres) -> Uuid {
    let id = Uuid::new_v4();
    let email = format!("testuser-{}@example.com", id);
    sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO users (id, last_name, first_name, email) VALUES ($1, $2, $3, $4) RETURNING id",
    )
    .bind(id)
    .bind("Test")
    .bind("User")
    .bind(email)
    .fetch_one(db.pool())
    .await
    .expect("failed to create test user")
}

pub async fn assert_create_api_key(mut db: Postgres) {
    let owner = create_test_user(&db).await;
    let perms = serde_json::json!(["UploadFile"]);
    let key = db
        .create_api_key(owner, "my-key".into(), "abc123hash".into(), perms)
        .await
        .expect("create_api_key failed");

    assert_eq!(key.owner(), owner, "owner mismatch: got {}", key.owner());
    assert_eq!(key.name(), "my-key", "name mismatch: got {}", key.name());
    assert_eq!(key.hash(), "abc123hash", "hash mismatch: got {}", key.hash());
}

pub async fn assert_read_api_key_by_hash(mut db: Postgres) {
    let owner = create_test_user(&db).await;
    let hash = format!("readhash-{}", Uuid::new_v4());
    db.create_api_key(owner, "read-key".into(), hash.clone(), serde_json::json!([]))
        .await
        .expect("create failed");

    let found = db
        .read_api_key_by_hash(&hash)
        .await
        .expect("read_api_key_by_hash failed");

    assert_eq!(found.hash(), hash, "hash mismatch: got {}", found.hash());
    assert_eq!(found.owner(), owner, "owner mismatch: got {}", found.owner());
}

pub async fn assert_delete_api_key(mut db: Postgres) {
    let owner = create_test_user(&db).await;
    let hash = format!("delhash-{}", Uuid::new_v4());
    let key = db
        .create_api_key(owner, "del-key".into(), hash.clone(), serde_json::json!([]))
        .await
        .expect("create failed");

    let deleted = db.delete_api_key(key.id()).await.expect("delete failed");
    assert!(deleted, "expected delete to return true, got false");

    let not_found = db.read_api_key_by_hash(&hash).await;
    assert!(
        not_found.is_err(),
        "key should be gone after delete, but read succeeded"
    );
}

pub async fn assert_delete_api_key_nonexistent(mut db: Postgres) {
    let result = db.delete_api_key(Uuid::new_v4()).await;
    match result {
        Ok(false) => {}
        Ok(true) => panic!("delete of nonexistent key returned true"),
        Err(e) => panic!("delete of nonexistent key returned error: {e}"),
    }
}
