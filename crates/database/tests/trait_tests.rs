use database::{
    Database,
    error::DatabaseError,
    models::{NewFile, User},
};
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

    #[test_trait]
    async fn create_directory_at_the_root(db: &mut impl Database) {
        let owner = create_test_user(db).await;

        let directory = db
            .create_directory(owner, None, "photos".into())
            .await
            .expect("create_directory failed");

        assert_eq!(
            directory.owner, owner,
            "owner mismatch: got {}",
            directory.owner
        );
        assert_eq!(
            directory.parent_id, None,
            "a root directory must have no parent, got {:?}",
            directory.parent_id
        );
        assert_eq!(
            directory.name, "photos",
            "name mismatch: got {}",
            directory.name
        );
    }

    #[test_trait]
    async fn read_directory_unknown(db: &mut impl Database) {
        let id = Uuid::new_v4();

        let result = db.read_directory(id).await;

        match result {
            Err(e) if matches!(*e, DatabaseError::NotFound(_)) => {}
            Err(e) => {
                panic!("reading an unknown directory must answer NotFound, got {e:?}")
            }
            Ok(directory) => {
                panic!("reading an unknown directory returned {}", directory.id)
            }
        }
    }

    #[test_trait]
    async fn read_child_directories_lists_only_direct_children(db: &mut impl Database) {
        let owner = create_test_user(db).await;
        let root = create_directory(db, owner, None, "root").await;
        let child = create_directory(db, owner, Some(root), "alpha").await;
        create_directory(db, owner, Some(child), "grandchild").await;
        create_directory(db, owner, None, "unrelated").await;

        let children = db
            .read_child_directories(root)
            .await
            .expect("read_child_directories failed");

        let names: Vec<&str> = children.iter().map(|d| d.name.as_str()).collect();
        assert_eq!(
            names,
            vec!["alpha"],
            "expected only the direct child, got {names:?}"
        );
    }

    #[test_trait]
    async fn read_root_directories_is_scoped_to_the_owner(db: &mut impl Database) {
        let owner = create_test_user(db).await;
        let stranger = create_test_user(db).await;
        create_directory(db, owner, None, "mine").await;
        create_directory(db, stranger, None, "theirs").await;

        let roots = db
            .read_root_directories(owner)
            .await
            .expect("read_root_directories failed");

        let names: Vec<&str> = roots.iter().map(|d| d.name.as_str()).collect();
        assert_eq!(
            names,
            vec!["mine"],
            "another owner's directory leaked into the listing, got {names:?}"
        );
    }

    #[test_trait]
    async fn read_directory_ancestors_walks_up_to_the_root(db: &mut impl Database) {
        let owner = create_test_user(db).await;
        let root = create_directory(db, owner, None, "root").await;
        let middle = create_directory(db, owner, Some(root), "middle").await;
        let leaf = create_directory(db, owner, Some(middle), "leaf").await;

        let ancestors = db
            .read_directory_ancestors(leaf)
            .await
            .expect("read_directory_ancestors failed");

        let ids: Vec<Uuid> = ancestors.iter().map(|d| d.id).collect();
        assert_eq!(
            ids,
            vec![leaf, middle, root],
            "expected the chain closest first, got {ids:?}"
        );
    }

    #[test_trait]
    async fn read_directory_ancestors_of_an_unknown_directory_is_empty(
        db: &mut impl Database,
    ) {
        let ancestors = db
            .read_directory_ancestors(Uuid::new_v4())
            .await
            .expect("read_directory_ancestors failed");

        assert!(
            ancestors.is_empty(),
            "an unknown directory must have no ancestors, got {} row(s)",
            ancestors.len()
        );
    }

    #[test_trait]
    async fn update_directory_renames_without_moving(db: &mut impl Database) {
        let owner = create_test_user(db).await;
        let parent = create_directory(db, owner, None, "parent").await;
        let child = create_directory(db, owner, Some(parent), "before").await;

        let updated = db
            .update_directory(child, Some("after".into()), None)
            .await
            .expect("update_directory failed");

        assert_eq!(updated.name, "after", "name mismatch: got {}", updated.name);
        assert_eq!(
            updated.parent_id,
            Some(parent),
            "an omitted parent must be left alone, got {:?}",
            updated.parent_id
        );
    }

    #[test_trait]
    async fn update_directory_moves_to_the_root(db: &mut impl Database) {
        let owner = create_test_user(db).await;
        let parent = create_directory(db, owner, None, "parent").await;
        let child = create_directory(db, owner, Some(parent), "child").await;

        let updated = db
            .update_directory(child, None, Some(None))
            .await
            .expect("update_directory failed");

        assert_eq!(
            updated.parent_id, None,
            "Some(None) must move the directory to the root, got {:?}",
            updated.parent_id
        );
        assert_eq!(
            updated.name, "child",
            "an omitted name must be left alone, got {}",
            updated.name
        );
    }

    #[test_trait]
    async fn update_directory_unknown(db: &mut impl Database) {
        let id = Uuid::new_v4();

        let result = db.update_directory(id, Some("nope".into()), None).await;

        match result {
            Err(e) if matches!(*e, DatabaseError::NotFound(_)) => {}
            Err(e) => {
                panic!("updating an unknown directory must answer NotFound, got {e:?}")
            }
            Ok(directory) => {
                panic!("updating an unknown directory returned {}", directory.id)
            }
        }
    }

    #[test_trait]
    async fn delete_directory_takes_its_subtree_with_it(db: &mut impl Database) {
        let owner = create_test_user(db).await;
        let root = create_directory(db, owner, None, "root").await;
        let child = create_directory(db, owner, Some(root), "child").await;

        let deleted = db.delete_directory(root).await.expect("delete failed");

        assert!(deleted, "expected delete to return true, got false");
        let orphan = db.read_directory(child).await;
        assert!(
            orphan.is_err(),
            "the child must cascade away with its parent, but the read succeeded"
        );
    }

    #[test_trait]
    async fn delete_directory_nonexistent(db: &mut impl Database) {
        let result = db.delete_directory(Uuid::new_v4()).await;

        match result {
            Ok(false) => {}
            Ok(true) => panic!("delete of a nonexistent directory returned true"),
            Err(e) => panic!("delete of a nonexistent directory returned error: {e}"),
        }
    }

    #[test_trait]
    async fn create_file_keeps_every_crypto_field(db: &mut impl Database) {
        let owner = create_test_user(db).await;
        let id = Uuid::now_v7();

        let file = db
            .create_file(NewFile {
                id,
                owner,
                parent_id: None,
                name: "notes.txt".into(),
                storage_key: format!("files/{id}/content"),
                mime_type: "text/plain".into(),
                size_bytes: 1024,
                stored_size_bytes: 512,
                is_compressed: true,
                encrypted_dek: vec![1, 2, 3],
                dek_nonce: vec![4, 5, 6],
                content_nonce: vec![7, 8, 9],
                thumbnail_storage_key: None,
                thumbnail_nonce: None,
            })
            .await
            .expect("create_file failed");

        assert_eq!(file.id, id, "the caller's id must be used, got {}", file.id);
        assert_eq!(
            file.encrypted_dek,
            vec![1, 2, 3],
            "encrypted_dek mismatch: got {:?}",
            file.encrypted_dek
        );
        assert_eq!(
            file.content_nonce,
            vec![7, 8, 9],
            "content_nonce mismatch: got {:?}",
            file.content_nonce
        );
        assert!(
            file.is_compressed,
            "is_compressed must survive the round trip, got false"
        );
        assert_eq!(
            file.stored_size_bytes, 512,
            "stored_size_bytes mismatch: got {}",
            file.stored_size_bytes
        );
    }

    #[test_trait]
    async fn read_file_unknown(db: &mut impl Database) {
        let result = db.read_file(Uuid::new_v4()).await;

        match result {
            Err(e) if matches!(*e, DatabaseError::NotFound(_)) => {}
            Err(e) => panic!("reading an unknown file must answer NotFound, got {e:?}"),
            Ok(file) => panic!("reading an unknown file returned {}", file.id),
        }
    }

    #[test_trait]
    async fn read_files_by_parent_lists_that_directory_only(db: &mut impl Database) {
        let owner = create_test_user(db).await;
        let directory = create_directory(db, owner, None, "docs").await;
        let other = create_directory(db, owner, None, "other").await;
        create_file(db, owner, Some(directory), "inside.txt").await;
        create_file(db, owner, Some(other), "elsewhere.txt").await;
        create_file(db, owner, None, "at-root.txt").await;

        let files = db
            .read_files_by_parent(directory)
            .await
            .expect("read_files_by_parent failed");

        let names: Vec<&str> = files.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(
            names,
            vec!["inside.txt"],
            "expected only the directory's own file, got {names:?}"
        );
    }

    #[test_trait]
    async fn read_root_files_is_scoped_to_the_owner(db: &mut impl Database) {
        let owner = create_test_user(db).await;
        let stranger = create_test_user(db).await;
        create_file(db, owner, None, "mine.txt").await;
        create_file(db, stranger, None, "theirs.txt").await;

        let files = db
            .read_root_files(owner)
            .await
            .expect("read_root_files failed");

        let names: Vec<&str> = files.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(
            names,
            vec!["mine.txt"],
            "another owner's file leaked into the listing, got {names:?}"
        );
    }

    #[test_trait]
    async fn update_file_renames_and_moves(db: &mut impl Database) {
        let owner = create_test_user(db).await;
        let destination = create_directory(db, owner, None, "destination").await;
        let file = create_file(db, owner, None, "before.txt").await;

        let updated = db
            .update_file(file, Some("after.txt".into()), Some(Some(destination)))
            .await
            .expect("update_file failed");

        assert_eq!(
            updated.name, "after.txt",
            "name mismatch: got {}",
            updated.name
        );
        assert_eq!(
            updated.parent_id,
            Some(destination),
            "the file must sit under the destination, got {:?}",
            updated.parent_id
        );
    }

    #[test_trait]
    async fn update_file_unknown(db: &mut impl Database) {
        let result = db
            .update_file(Uuid::new_v4(), Some("nope.txt".into()), None)
            .await;

        match result {
            Err(e) if matches!(*e, DatabaseError::NotFound(_)) => {}
            Err(e) => panic!("updating an unknown file must answer NotFound, got {e:?}"),
            Ok(file) => panic!("updating an unknown file returned {}", file.id),
        }
    }

    #[test_trait]
    async fn delete_file(db: &mut impl Database) {
        let owner = create_test_user(db).await;
        let file = create_file(db, owner, None, "doomed.txt").await;

        let deleted = db.delete_file(file).await.expect("delete failed");

        assert!(deleted, "expected delete to return true, got false");
        let gone = db.read_file(file).await;
        assert!(
            gone.is_err(),
            "the file must be gone after delete, but the read succeeded"
        );
    }

    #[test_trait]
    async fn delete_file_nonexistent(db: &mut impl Database) {
        let result = db.delete_file(Uuid::new_v4()).await;

        match result {
            Ok(false) => {}
            Ok(true) => panic!("delete of a nonexistent file returned true"),
            Err(e) => panic!("delete of a nonexistent file returned error: {e}"),
        }
    }

    #[test_trait]
    async fn upsert_directory_permission_replaces_the_level(db: &mut impl Database) {
        let owner = create_test_user(db).await;
        let grantee = create_test_user(db).await;
        let directory = create_directory(db, owner, None, "shared").await;

        let first = db
            .upsert_directory_permission(directory, grantee, "viewer", owner)
            .await
            .expect("first upsert failed");
        let second = db
            .upsert_directory_permission(directory, grantee, "manager", owner)
            .await
            .expect("second upsert failed");

        assert_eq!(
            first.id, second.id,
            "the upsert must reuse the row, got {} then {}",
            first.id, second.id
        );
        assert_eq!(
            second.permission_level, "manager",
            "the level must be replaced, got {}",
            second.permission_level
        );

        let all = db
            .read_directory_permissions(directory)
            .await
            .expect("read_directory_permissions failed");
        assert_eq!(
            all.len(),
            1,
            "the upsert must not add a second row, got {} row(s)",
            all.len()
        );
    }

    #[test_trait]
    async fn read_directory_permission_of_a_stranger_is_none(db: &mut impl Database) {
        let owner = create_test_user(db).await;
        let stranger = create_test_user(db).await;
        let directory = create_directory(db, owner, None, "private").await;

        let permission = db
            .read_directory_permission(directory, stranger)
            .await
            .expect("read_directory_permission failed");

        assert!(
            permission.is_none(),
            "a stranger must hold no grant, got {permission:?}"
        );
    }

    #[test_trait]
    async fn read_directory_permissions_for_grantee_batches_a_chain(
        db: &mut impl Database,
    ) {
        let owner = create_test_user(db).await;
        let grantee = create_test_user(db).await;
        let stranger = create_test_user(db).await;
        let root = create_directory(db, owner, None, "root").await;
        let leaf = create_directory(db, owner, Some(root), "leaf").await;
        db.upsert_directory_permission(root, grantee, "editor", owner)
            .await
            .expect("upsert on root failed");
        db.upsert_directory_permission(leaf, stranger, "viewer", owner)
            .await
            .expect("upsert for the stranger failed");

        let grants = db
            .read_directory_permissions_for_grantee(&[leaf, root], grantee)
            .await
            .expect("read_directory_permissions_for_grantee failed");

        assert_eq!(
            grants.len(),
            1,
            "only the grantee's own grant must come back, got {} row(s)",
            grants.len()
        );
        assert_eq!(
            grants[0].directory_id, root,
            "the grant must be the one on the root, got {}",
            grants[0].directory_id
        );
    }

    #[test_trait]
    async fn delete_directory_permission(db: &mut impl Database) {
        let owner = create_test_user(db).await;
        let grantee = create_test_user(db).await;
        let directory = create_directory(db, owner, None, "shared").await;
        db.upsert_directory_permission(directory, grantee, "viewer", owner)
            .await
            .expect("upsert failed");

        let deleted = db
            .delete_directory_permission(directory, grantee)
            .await
            .expect("delete failed");

        assert!(deleted, "expected delete to return true, got false");
        let gone = db
            .read_directory_permission(directory, grantee)
            .await
            .expect("read_directory_permission failed");
        assert!(
            gone.is_none(),
            "the grant must be gone after delete, got {gone:?}"
        );
    }

    #[test_trait]
    async fn delete_directory_permission_nonexistent(db: &mut impl Database) {
        let owner = create_test_user(db).await;
        let grantee = create_test_user(db).await;
        let directory = create_directory(db, owner, None, "shared").await;

        let result = db.delete_directory_permission(directory, grantee).await;

        match result {
            Ok(false) => {}
            Ok(true) => panic!("delete of a nonexistent grant returned true"),
            Err(e) => panic!("delete of a nonexistent grant returned error: {e}"),
        }
    }

    #[test_trait]
    async fn upsert_file_permission_replaces_the_level(db: &mut impl Database) {
        let owner = create_test_user(db).await;
        let grantee = create_test_user(db).await;
        let file = create_file(db, owner, None, "shared.txt").await;

        let first = db
            .upsert_file_permission(file, grantee, "viewer", owner)
            .await
            .expect("first upsert failed");
        let second = db
            .upsert_file_permission(file, grantee, "editor", owner)
            .await
            .expect("second upsert failed");

        assert_eq!(
            first.id, second.id,
            "the upsert must reuse the row, got {} then {}",
            first.id, second.id
        );
        assert_eq!(
            second.permission_level, "editor",
            "the level must be replaced, got {}",
            second.permission_level
        );
    }

    #[test_trait]
    async fn read_file_permissions_lists_that_file_only(db: &mut impl Database) {
        let owner = create_test_user(db).await;
        let grantee = create_test_user(db).await;
        let file = create_file(db, owner, None, "shared.txt").await;
        let other = create_file(db, owner, None, "other.txt").await;
        db.upsert_file_permission(file, grantee, "viewer", owner)
            .await
            .expect("upsert failed");
        db.upsert_file_permission(other, grantee, "manager", owner)
            .await
            .expect("upsert on the other file failed");

        let grants = db
            .read_file_permissions(file)
            .await
            .expect("read_file_permissions failed");

        assert_eq!(
            grants.len(),
            1,
            "only the file's own grant must come back, got {} row(s)",
            grants.len()
        );
        assert_eq!(
            grants[0].permission_level, "viewer",
            "level mismatch: got {}",
            grants[0].permission_level
        );
    }

    #[test_trait]
    async fn delete_file_permission(db: &mut impl Database) {
        let owner = create_test_user(db).await;
        let grantee = create_test_user(db).await;
        let file = create_file(db, owner, None, "shared.txt").await;
        db.upsert_file_permission(file, grantee, "viewer", owner)
            .await
            .expect("upsert failed");

        let deleted = db
            .delete_file_permission(file, grantee)
            .await
            .expect("delete failed");

        assert!(deleted, "expected delete to return true, got false");
        let gone = db
            .read_file_permission(file, grantee)
            .await
            .expect("read_file_permission failed");
        assert!(
            gone.is_none(),
            "the grant must be gone after delete, got {gone:?}"
        );
    }

    #[test_trait]
    async fn deleting_a_file_takes_its_grants_with_it(db: &mut impl Database) {
        let owner = create_test_user(db).await;
        let grantee = create_test_user(db).await;
        let file = create_file(db, owner, None, "shared.txt").await;
        db.upsert_file_permission(file, grantee, "viewer", owner)
            .await
            .expect("upsert failed");

        db.delete_file(file).await.expect("delete failed");

        let grants = db
            .read_file_permissions(file)
            .await
            .expect("read_file_permissions failed");
        assert!(
            grants.is_empty(),
            "the grants must cascade away with the file, got {} row(s)",
            grants.len()
        );
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

/// Creates a directory owned by `owner` and returns its id.
async fn create_directory(
    db: &mut impl Database,
    owner: Uuid,
    parent_id: Option<Uuid>,
    name: &str,
) -> Uuid {
    db.create_directory(owner, parent_id, name.to_string())
        .await
        .expect("failed to create a directory")
        .id
}

/// Creates a file owned by `owner` and returns its id.
///
/// Storage keys are UNIQUE, so each call derives its own from the row id rather
/// than sharing one: trials run in parallel against the same database.
async fn create_file(
    db: &mut impl Database,
    owner: Uuid,
    parent_id: Option<Uuid>,
    name: &str,
) -> Uuid {
    let id = Uuid::now_v7();
    db.create_file(NewFile {
        id,
        owner,
        parent_id,
        name: name.to_string(),
        storage_key: format!("files/{id}/content"),
        mime_type: "application/octet-stream".to_string(),
        size_bytes: 0,
        stored_size_bytes: 0,
        is_compressed: false,
        encrypted_dek: Vec::new(),
        dek_nonce: Vec::new(),
        content_nonce: Vec::new(),
        thumbnail_storage_key: None,
        thumbnail_nonce: None,
    })
    .await
    .expect("failed to create a file")
    .id
}
