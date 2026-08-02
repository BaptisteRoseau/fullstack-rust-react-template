use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::error::CoreError;
use crate::models::api_key_from_db;
use database::Database;

pub async fn create_api_key(
    db: &mut dyn Database,
    owner: Uuid,
    name: String,
    permissions: Vec<rbac::Permissions>,
) -> Result<(String, models::ApiKey), CoreError> {
    let permissions_json = serde_json::to_value(&permissions)?;
    loop {
        let raw_key = generate_random_key();
        let hash = hex_sha256(&raw_key);
        match db
            .create_api_key(owner, name.clone(), hash, permissions_json.clone())
            .await
        {
            Ok(db_key) => {
                let api_key = api_key_from_db(db_key)?;
                return Ok((raw_key, api_key));
            }
            Err(e) if e.is_hash_collision() => continue,
            Err(e) => return Err(CoreError::DatabaseError(e)),
        }
    }
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

// #[cfg(test)]
// #[path = "_tests/test_api_key.rs"]
// mod tests;
