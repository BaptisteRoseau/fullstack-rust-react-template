use sha2::{Digest, Sha256};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::access::ensure_owner;
use crate::error::CoreError;
use crate::models::api_key_from_db;
use authenticator::Caller;
use database::Database;

/// Lists the keys owned by the caller, newest first.
pub async fn list_api_keys(
    db: &Arc<RwLock<dyn Database>>,
    caller: &Caller,
) -> Result<Vec<models::ApiKey>, CoreError> {
    let db_keys = db.read().await.read_api_keys_by_owner(caller.id).await?;
    db_keys.into_iter().map(api_key_from_db).collect()
}

/// Creates a key owned by the caller. Returns the raw key, which is never stored.
pub async fn create_api_key(
    db: &Arc<RwLock<dyn Database>>,
    caller: &Caller,
    name: String,
    permissions: Vec<rbac::Permissions>,
) -> Result<(String, models::ApiKey), CoreError> {
    let permissions_json = serde_json::to_value(&permissions)?;
    loop {
        let raw_key = generate_random_key();
        let hash = hex_sha256(&raw_key);
        let created = db
            .write()
            .await
            .create_api_key(caller.id, name.clone(), hash, permissions_json.clone())
            .await;
        match created {
            Ok(db_key) => {
                let api_key = api_key_from_db(db_key)?;
                return Ok((raw_key, api_key));
            }
            Err(e) if e.is_hash_collision() => continue,
            Err(e) => return Err(CoreError::DatabaseError(e)),
        }
    }
}

/// Reads one key. A key the caller does not own answers `NotFound`.
pub async fn get_api_key(
    db: &Arc<RwLock<dyn Database>>,
    caller: &Caller,
    id: Uuid,
) -> Result<models::ApiKey, CoreError> {
    let db_key = db
        .read()
        .await
        .read_api_key_by_id(id)
        .await
        .map_err(|error| CoreError::from_database(*error))?;
    ensure_owner(caller, db_key.owner, id)?;
    api_key_from_db(db_key)
}

/// Deletes one key. A key the caller does not own answers `NotFound`.
pub async fn delete_api_key(
    db: &Arc<RwLock<dyn Database>>,
    caller: &Caller,
    id: Uuid,
) -> Result<(), CoreError> {
    let db_key = db
        .read()
        .await
        .read_api_key_by_id(id)
        .await
        .map_err(|error| CoreError::from_database(*error))?;
    ensure_owner(caller, db_key.owner, id)?;
    db.write().await.delete_api_key(id).await?;
    Ok(())
}

fn generate_random_key() -> String {
    rand::random::<[u8; 32]>()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn hex_sha256(input: &str) -> String {
    Sha256::digest(input.as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

// test_utils::tests_file!("_tests/test_api_key.rs");
