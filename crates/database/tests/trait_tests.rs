use database::{Database, error::DatabaseError, models::User};
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
        assert_eq!(key.hash, "abc123hash", "hash mismatch: got {}", key.hash);
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
        assert_eq!(found.owner, owner, "owner mismatch: got {}", found.owner);
    }

    #[test_trait]
    async fn read_api_keys_by_owner(db: &mut impl Database) {
        let owner = create_test_user(db).await;
        let stranger = create_test_user(db).await;
        let older = create_owned_api_key(db, owner, "older").await;
        let newer = create_owned_api_key(db, owner, "newer").await;
        create_owned_api_key(db, stranger, "stranger").await;

        let keys = db
            .read_api_keys_by_owner(owner)
            .await
            .expect("read_api_keys_by_owner failed");

        let ids: Vec<Uuid> = keys.iter().map(|key| key.id).collect();
        assert_eq!(
            ids,
            vec![newer, older],
            "expected the owner's keys newest first, got {ids:?}"
        );
    }

    #[test_trait]
    async fn read_api_keys_by_owner_without_any(db: &mut impl Database) {
        let owner = create_test_user(db).await;

        let keys = db
            .read_api_keys_by_owner(owner)
            .await
            .expect("read_api_keys_by_owner failed");

        assert!(
            keys.is_empty(),
            "an owner with no key must list nothing, got {} key(s)",
            keys.len()
        );
    }

    #[test_trait]
    async fn update_user(db: &mut impl Database) {
        let id = create_test_user(db).await;

        let updated = db
            .update_user(
                User::build_patch(id)
                    .set_first_name("Ada")
                    .set_last_name("Lovelace"),
            )
            .await
            .expect("update_user failed");

        assert_eq!(
            updated.first_name, "Ada",
            "first_name mismatch: got {}",
            updated.first_name
        );
        assert_eq!(
            updated.last_name, "Lovelace",
            "last_name mismatch: got {}",
            updated.last_name
        );

        let stored = db.read_user(id).await.expect("read_user failed");
        assert_eq!(
            stored.first_name, "Ada",
            "the update was not persisted, read back {}",
            stored.first_name
        );
    }

    #[test_trait]
    async fn update_user_unknown(db: &mut impl Database) {
        let id = Uuid::new_v4();

        let result = db
            .update_user(User::build_patch(id).set_first_name("Ada"))
            .await;

        match result {
            Err(e) if matches!(*e, DatabaseError::NotFound(_)) => {}
            Err(e) => panic!("updating an unknown user must answer NotFound, got {e:?}"),
            Ok(user) => panic!("updating an unknown user returned {}", user.id),
        }
    }

    #[test_trait]
    async fn register_keeps_the_local_name(db: &mut impl Database) {
        let id = Uuid::new_v4();
        let username = format!("testuser-{id}");
        let email = format!("{username}@example.com");
        db.register(id, username.clone(), "Ada".into(), "Lovelace".into(), email)
            .await
            .expect("first register failed");

        let new_email = format!("{username}@example.org");
        let synced = db
            .register(
                id,
                username,
                "Renamed".into(),
                "Upstream".into(),
                new_email.clone(),
            )
            .await
            .expect("second register failed");

        assert_eq!(
            synced.first_name, "Ada",
            "the provider must not overwrite a locally owned first_name, got {}",
            synced.first_name
        );
        assert_eq!(
            synced.last_name, "Lovelace",
            "the provider must not overwrite a locally owned last_name, got {}",
            synced.last_name
        );
        assert_eq!(
            synced.email, new_email,
            "email must be re-synced from the provider, got {}",
            synced.email
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

/// Creates a key owned by `owner` and returns its id.
///
/// Hashes are UNIQUE, so each call derives its own rather than sharing one.
async fn create_owned_api_key(db: &mut impl Database, owner: Uuid, name: &str) -> Uuid {
    db.create_api_key(
        owner,
        name.to_string(),
        format!("hash-{}", Uuid::new_v4()),
        serde_json::json!([]),
    )
    .await
    .expect("failed to create an api key")
    .id
}
