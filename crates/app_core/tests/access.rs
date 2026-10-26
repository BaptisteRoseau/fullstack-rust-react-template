use std::collections::HashSet;

use app_core::error::CoreError;
use database::testing::MockDatabase;
use rbac::UserPermissions;
use uuid::Uuid;

fn caller(id: Uuid) -> UserPermissions {
    UserPermissions::new(id, HashSet::new(), HashSet::new())
}

async fn registered_user(db: &mut MockDatabase) -> Uuid {
    let id = Uuid::new_v4();
    app_core::user::register(
        db,
        id,
        "jdoe".to_string(),
        "John".to_string(),
        "Doe".to_string(),
        "jdoe@example.com".to_string(),
    )
    .await
    .expect("registering a user in the mock database");
    id
}

#[tokio::test]
async fn test_read_profile_allows_the_owner() {
    let mut db = MockDatabase::default();
    let owner = registered_user(&mut db).await;

    let profile = app_core::user::read_profile(&db, &caller(owner), owner).await;

    assert!(
        matches!(profile, Ok(Some(ref user)) if user.id == owner),
        "the owner must read their own profile, got {profile:?}"
    );
}

#[tokio::test]
async fn test_read_profile_hides_other_users() {
    let mut db = MockDatabase::default();
    let owner = registered_user(&mut db).await;
    let stranger = Uuid::new_v4();

    let profile = app_core::user::read_profile(&db, &caller(stranger), owner).await;

    assert!(
        matches!(profile, Err(CoreError::NotFound(_))),
        "another user's profile must answer NotFound, got {profile:?}"
    );
}

#[tokio::test]
async fn test_update_profile_refuses_other_users() {
    let mut db = MockDatabase::default();
    let owner = registered_user(&mut db).await;
    let stranger = Uuid::new_v4();

    let result = app_core::user::update_profile(
        &mut db,
        &caller(stranger),
        owner,
        "Mallory".to_string(),
        "Evil".to_string(),
    )
    .await;

    assert!(
        matches!(result, Err(CoreError::NotFound(_))),
        "another user's profile must not be changed, got {result:?}"
    );
    let first_name = &db.users[&owner].first_name;
    assert_eq!(first_name, "John", "the profile changed to {first_name}");
}

#[tokio::test]
async fn test_get_api_key_allows_the_owner() {
    let mut db = MockDatabase::default();
    let owner = caller(Uuid::new_v4());
    let (_, created) =
        app_core::api_key::create_api_key(&mut db, &owner, "key".to_string(), vec![])
            .await
            .expect("creating a key");

    let read = app_core::api_key::get_api_key(&db, &owner, created.id).await;

    assert!(
        matches!(read, Ok(ref key) if key.id == created.id),
        "the owner must read their own key, got {read:?}"
    );
}

#[tokio::test]
async fn test_get_api_key_hides_other_users_keys() {
    let mut db = MockDatabase::default();
    let owner = caller(Uuid::new_v4());
    let stranger = caller(Uuid::new_v4());
    let (_, created) =
        app_core::api_key::create_api_key(&mut db, &owner, "key".to_string(), vec![])
            .await
            .expect("creating a key");

    let read = app_core::api_key::get_api_key(&db, &stranger, created.id).await;

    assert!(
        matches!(read, Err(CoreError::NotFound(_))),
        "another user's key must answer NotFound, got {read:?}"
    );
}

#[tokio::test]
async fn test_get_api_key_unknown_id_is_not_found() {
    let db = MockDatabase::default();
    let unknown = Uuid::new_v4();

    let read =
        app_core::api_key::get_api_key(&db, &caller(Uuid::new_v4()), unknown).await;

    assert!(
        matches!(read, Err(CoreError::NotFound(_))),
        "an unknown key must answer NotFound, got {read:?}"
    );
}

#[tokio::test]
async fn test_delete_api_key_refuses_other_users() {
    let mut db = MockDatabase::default();
    let owner = caller(Uuid::new_v4());
    let stranger = caller(Uuid::new_v4());
    let (_, created) =
        app_core::api_key::create_api_key(&mut db, &owner, "key".to_string(), vec![])
            .await
            .expect("creating a key");

    let result = app_core::api_key::delete_api_key(&mut db, &stranger, created.id).await;

    assert!(
        matches!(result, Err(CoreError::NotFound(_))),
        "another user's key must not be deleted, got {result:?}"
    );
    assert!(
        db.api_keys_by_id.contains_key(&created.id),
        "the key {} was deleted by a stranger",
        created.id
    );
}

#[tokio::test]
async fn test_delete_api_key_allows_the_owner() {
    let mut db = MockDatabase::default();
    let owner = caller(Uuid::new_v4());
    let (_, created) =
        app_core::api_key::create_api_key(&mut db, &owner, "key".to_string(), vec![])
            .await
            .expect("creating a key");

    let result = app_core::api_key::delete_api_key(&mut db, &owner, created.id).await;

    assert!(
        result.is_ok(),
        "the owner must delete their own key, got {result:?}"
    );
    assert!(
        !db.api_keys_by_id.contains_key(&created.id),
        "the key {} is still stored",
        created.id
    );
}

#[tokio::test]
async fn test_list_api_keys_only_lists_the_callers_keys() {
    let mut db = MockDatabase::default();
    let owner = caller(Uuid::new_v4());
    let stranger = caller(Uuid::new_v4());
    app_core::api_key::create_api_key(&mut db, &owner, "key".to_string(), vec![])
        .await
        .expect("creating a key");

    let listed = app_core::api_key::list_api_keys(&db, &stranger)
        .await
        .expect("listing keys");

    assert!(listed.is_empty(), "a stranger listed {listed:?}");
}
