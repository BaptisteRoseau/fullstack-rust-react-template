use std::sync::Arc;

use app_core::error::CoreError;
use authenticator::backends::Keycloak;
use authenticator::{Authenticator, Caller};
use cache::Cache;
use cache::backends::hash_map::HashMapCache;
use database::Database;
use database::testing::MockDatabase;
use tokio::sync::RwLock;
use uuid::Uuid;

/// The mock, to inspect afterwards, and the same database as `app_core` takes it.
fn databases() -> (Arc<RwLock<MockDatabase>>, Arc<RwLock<dyn Database>>) {
    let mock = Arc::new(RwLock::new(MockDatabase::default()));
    let db: Arc<RwLock<dyn Database>> = mock.clone();
    (mock, db)
}

/// The real Keycloak backend over the same database, built offline: only API keys are used.
async fn authenticator(
    db: &Arc<RwLock<dyn Database>>,
) -> (Arc<RwLock<dyn Cache>>, Arc<RwLock<dyn Authenticator>>) {
    let cache: Arc<RwLock<dyn Cache>> = Arc::new(RwLock::new(HashMapCache::default()));
    let mut config = config::testing::test_config();
    config.authenticator.issuer_url = UNREACHABLE_ISSUER.to_string();
    let keycloak = Keycloak::try_new(&config, Arc::clone(&cache), Arc::clone(db))
        .await
        .expect("building the keycloak authenticator");
    (cache, Arc::new(RwLock::new(keycloak)))
}

/// Port 9 is the discard port: nothing listens there, so the JWKS fetch fails at once.
const UNREACHABLE_ISSUER: &str = "http://127.0.0.1:9/realms/test";

fn caller(id: Uuid) -> Caller {
    Caller {
        id,
        realm: "test".to_string(),
    }
}

async fn registered_user(db: &Arc<RwLock<dyn Database>>) -> Uuid {
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
    let (_, db) = databases();
    let owner = registered_user(&db).await;

    let profile = app_core::user::read_profile(&db, &caller(owner), owner).await;

    assert!(
        matches!(profile, Ok(Some(ref user)) if user.id == owner),
        "the owner must read their own profile, got {profile:?}"
    );
}

#[tokio::test]
async fn test_read_profile_hides_other_users() {
    let (_, db) = databases();
    let owner = registered_user(&db).await;
    let stranger = Uuid::new_v4();

    let profile = app_core::user::read_profile(&db, &caller(stranger), owner).await;

    assert!(
        matches!(profile, Err(CoreError::NotFound(_))),
        "another user's profile must answer NotFound, got {profile:?}"
    );
}

#[tokio::test]
async fn test_update_profile_refuses_other_users() {
    let (mock, db) = databases();
    let owner = registered_user(&db).await;
    let stranger = Uuid::new_v4();

    let result = app_core::user::update_profile(
        &db,
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
    let first_name = &mock.read().await.users[&owner].first_name;
    assert_eq!(first_name, "John", "the profile changed to {first_name}");
}

#[tokio::test]
async fn test_get_api_key_allows_the_owner() {
    let (_, db) = databases();
    let owner = caller(Uuid::new_v4());
    let (_, created) =
        app_core::api_key::create_api_key(&db, &owner, "key".to_string(), vec![])
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
    let (_, db) = databases();
    let owner = caller(Uuid::new_v4());
    let stranger = caller(Uuid::new_v4());
    let (_, created) =
        app_core::api_key::create_api_key(&db, &owner, "key".to_string(), vec![])
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
    let (_, db) = databases();
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
    let (mock, db) = databases();
    let owner = caller(Uuid::new_v4());
    let stranger = caller(Uuid::new_v4());
    let (_, created) =
        app_core::api_key::create_api_key(&db, &owner, "key".to_string(), vec![])
            .await
            .expect("creating a key");

    let (_, authenticator) = authenticator(&db).await;

    let result =
        app_core::api_key::delete_api_key(&db, &authenticator, &stranger, created.id)
            .await;

    assert!(
        matches!(result, Err(CoreError::NotFound(_))),
        "another user's key must not be deleted, got {result:?}"
    );
    assert!(
        mock.read().await.api_keys_by_id.contains_key(&created.id),
        "the key {} was deleted by a stranger",
        created.id
    );
}

#[tokio::test]
async fn test_delete_api_key_allows_the_owner() {
    let (mock, db) = databases();
    let owner = caller(Uuid::new_v4());
    let (_, created) =
        app_core::api_key::create_api_key(&db, &owner, "key".to_string(), vec![])
            .await
            .expect("creating a key");

    let (_, authenticator) = authenticator(&db).await;

    let result =
        app_core::api_key::delete_api_key(&db, &authenticator, &owner, created.id).await;

    assert!(
        result.is_ok(),
        "the owner must delete their own key, got {result:?}"
    );
    assert!(
        !mock.read().await.api_keys_by_id.contains_key(&created.id),
        "the key {} is still stored",
        created.id
    );
}

#[tokio::test]
async fn test_deleted_api_key_is_refused_at_once() {
    let (_, db) = databases();
    let (_, authenticator) = authenticator(&db).await;
    let owner = caller(Uuid::new_v4());
    let (raw_key, created) =
        app_core::api_key::create_api_key(&db, &owner, "key".to_string(), vec![])
            .await
            .expect("creating a key");
    authenticator
        .read()
        .await
        .validate(&raw_key)
        .await
        .expect("a fresh key validates, and its validation is now cached");

    app_core::api_key::delete_api_key(&db, &authenticator, &owner, created.id)
        .await
        .expect("the owner deletes their key");

    let result = authenticator.read().await.validate(&raw_key).await;
    assert!(
        result.is_err(),
        "a deleted key must be refused without waiting for the cache, got {result:?}"
    );
}

#[tokio::test]
async fn test_deleting_an_api_key_keeps_other_cache_entries() {
    let (mock, db) = databases();
    let (cache, authenticator) = authenticator(&db).await;
    let owner = caller(Uuid::new_v4());
    let (_, deleted) =
        app_core::api_key::create_api_key(&db, &owner, "deleted".to_string(), vec![])
            .await
            .expect("creating a key");
    let (kept_raw_key, kept) =
        app_core::api_key::create_api_key(&db, &owner, "kept".to_string(), vec![])
            .await
            .expect("creating a key");
    authenticator
        .read()
        .await
        .validate(&kept_raw_key)
        .await
        .expect("a fresh key validates, and its validation is now cached");
    let kept_hash = mock.read().await.api_keys_by_id[&kept.id].hash.clone();

    app_core::api_key::delete_api_key(&db, &authenticator, &owner, deleted.id)
        .await
        .expect("the owner deletes their key");

    let cached = cache.read().await.get(&kept_hash).await;
    assert!(
        matches!(cached, Ok(Some(_))),
        "deleting {} dropped the cache entry of {}, got {cached:?}",
        deleted.id,
        kept.id
    );
}

#[tokio::test]
async fn test_list_api_keys_only_lists_the_callers_keys() {
    let (_, db) = databases();
    let owner = caller(Uuid::new_v4());
    let stranger = caller(Uuid::new_v4());
    app_core::api_key::create_api_key(&db, &owner, "key".to_string(), vec![])
        .await
        .expect("creating a key");

    let listed = app_core::api_key::list_api_keys(&db, &stranger)
        .await
        .expect("listing keys");

    assert!(listed.is_empty(), "a stranger listed {listed:?}");
}
