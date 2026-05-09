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

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use async_trait::async_trait;
    use database::{
        Database,
        error::DatabaseError,
        models::{ApiKey as DbApiKey, User, UserPatch},
    };
    use uuid::Uuid;

    use super::create_api_key;

    struct MockDatabase {
        calls: Arc<Mutex<u32>>,
    }

    impl MockDatabase {
        fn new() -> Self {
            Self {
                calls: Arc::new(Mutex::new(0)),
            }
        }
    }

    #[async_trait]
    impl Database for MockDatabase {
        async fn create_user(
            &mut self,
            _patch: UserPatch,
        ) -> Result<User, Box<DatabaseError>> {
            unimplemented!()
        }
        async fn update_user(
            &mut self,
            _patch: UserPatch,
        ) -> Result<User, Box<DatabaseError>> {
            unimplemented!()
        }
        async fn read_user(&self, _uuid: Uuid) -> Result<User, Box<DatabaseError>> {
            unimplemented!()
        }
        async fn delete_user(
            &mut self,
            _uuid: Uuid,
        ) -> Result<bool, Box<DatabaseError>> {
            unimplemented!()
        }
        async fn read_api_key_by_id(
            &self,
            _id: Uuid,
        ) -> Result<DbApiKey, Box<DatabaseError>> {
            unimplemented!()
        }
        async fn read_api_key_by_hash(
            &self,
            _hash: &str,
        ) -> Result<DbApiKey, Box<DatabaseError>> {
            unimplemented!()
        }
        async fn delete_api_key(
            &mut self,
            _id: Uuid,
        ) -> Result<bool, Box<DatabaseError>> {
            unimplemented!()
        }

        async fn create_api_key(
            &mut self,
            _owner: Uuid,
            _name: String,
            _hash: String,
            _permissions: serde_json::Value,
        ) -> Result<DbApiKey, Box<DatabaseError>> {
            let mut calls = self.calls.lock().unwrap();
            *calls += 1;
            let call_number = *calls;
            drop(calls);

            if call_number == 1 {
                // Simulate hash collision on first call
                Err(Box::new(DatabaseError::HashCollision))
            } else {
                // Sentinel error to exit the retry loop cleanly
                Err(Box::new(DatabaseError::NotFound("test-sentinel".into())))
            }
        }
    }

    #[tokio::test]
    async fn test_hash_collision_retry() {
        let mut mock = MockDatabase::new();
        let calls = mock.calls.clone();

        let result = create_api_key(
            &mut mock,
            Uuid::new_v4(),
            "my key".into(),
            vec![],
        )
        .await;

        let total_calls = *calls.lock().unwrap();
        assert_eq!(total_calls, 2, "expected 2 DB calls (1 collision + 1 sentinel), got {total_calls}");
        assert!(result.is_err(), "expected error from sentinel, got Ok");
        // Should NOT be a hash collision error (that was retried)
        if let Err(crate::error::CoreError::DatabaseError(e)) = &result {
            assert!(
                !e.is_hash_collision(),
                "error should not be a hash collision after retry, got: {e:?}"
            );
        }
    }
}
