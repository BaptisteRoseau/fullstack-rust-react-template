use database::Database;
use test_trait::{test_trait, test_trait_suite};
use uuid::Uuid;

/// Integration tests for the Database trait, run against every backend.
///
/// When adding a test here:
/// - mark it `#[test_trait]` and take the subject as `&mut impl Database`; the
///   function name becomes the test name, and that is the only place it is written
/// - helpers are regular functions, left alone by the macro
#[test_trait_suite]
pub mod suite {
    use super::*;

    #[test_trait]
    async fn create_api_key(db: &mut impl Database) {
        let owner = create_test_user(db).await;
        let perms = serde_json::json!(["UploadFile"]);
        let key = db
            .create_api_key(owner, "my-key".into(), "abc123hash".into(), perms)
            .await
            .expect("create_api_key failed");

        assert_eq!(key.owner, owner, "owner mismatch: got {}", key.owner);
        assert_eq!(key.name, "my-key", "name mismatch: got {}", key.name);
        assert_eq!(
            key.hash,
            "abc123hash",
            "hash mismatch: got {}",
            key.hash
        );
    }

    #[test_trait]
    async fn read_api_key_by_hash(db: &mut impl Database) {
        let owner = create_test_user(db).await;
        let hash = format!("readhash-{}", Uuid::new_v4());
        db.create_api_key(
            owner,
            "read-key".into(),
            hash.clone(),
            serde_json::json!([]),
        )
        .await
        .expect("create failed");

        let found = db
            .read_api_key_by_hash(&hash)
            .await
            .expect("read_api_key_by_hash failed");

        assert_eq!(found.hash, hash, "hash mismatch: got {}", found.hash);
        assert_eq!(
            found.owner,
            owner,
            "owner mismatch: got {}",
            found.owner
        );
    }

    #[test_trait]
    async fn delete_api_key(db: &mut impl Database) {
        let owner = create_test_user(db).await;
        let hash = format!("delhash-{}", Uuid::new_v4());
        let key = db
            .create_api_key(owner, "del-key".into(), hash.clone(), serde_json::json!([]))
            .await
            .expect("create failed");

        let deleted = db.delete_api_key(key.id).await.expect("delete failed");
        assert!(deleted, "expected delete to return true, got false");

        let not_found = db.read_api_key_by_hash(&hash).await;
        assert!(
            not_found.is_err(),
            "key should be gone after delete, but read succeeded"
        );
    }

    #[test_trait]
    async fn delete_api_key_nonexistent(db: &mut impl Database) {
        let result = db.delete_api_key(Uuid::new_v4()).await;
        match result {
            Ok(false) => {}
            Ok(true) => panic!("delete of nonexistent key returned true"),
            Err(e) => panic!("delete of nonexistent key returned error: {e}"),
        }
    }
}

/// Creates a user to own the keys under test and returns its id.
///
/// Goes through `register` rather than raw SQL so the suite stays expressible in
/// terms of the trait alone. Uses a unique id, username and email per call: the
/// latter two are UNIQUE NOT NULL, so sharing them would make parallel trials
/// collide.
async fn create_test_user(db: &mut impl Database) -> Uuid {
    let id = Uuid::new_v4();
    let username = format!("testuser-{id}");
    let email = format!("{username}@example.com");

    db.register(id, username, "Test".into(), "User".into(), email)
        .await
        .expect("failed to create test user")
        .id
}
