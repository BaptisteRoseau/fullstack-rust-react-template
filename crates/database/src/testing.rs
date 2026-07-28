//! In-memory [`Database`] test double, shared by downstream crates' tests.
//!
//! Enable via the `test-utils` feature (add it under `[dev-dependencies]`, not
//! `[dependencies]`, so it never leaks into non-test builds).

use std::collections::HashMap;

use async_trait::async_trait;
use uuid::Uuid;

use crate::database::Database;
use crate::error::DatabaseError;
use crate::models::{ApiKey, User, UserPatch};

/// In-memory `Database` backed by plain `HashMap`s. Seed state through the
/// public fields (e.g. `MockDatabase { api_keys_by_hash, ..Default::default() }`)
/// before exercising the code under test.
///
/// `create_user`/`update_user` are intentionally left unimplemented: the real
/// `Postgres` implementation of both is currently broken (they never insert),
/// so a working mock here would silently diverge from production behavior.
#[derive(Default)]
pub struct MockDatabase {
    pub users: HashMap<Uuid, User>,
    pub api_keys_by_id: HashMap<Uuid, ApiKey>,
    pub api_keys_by_hash: HashMap<String, ApiKey>,
}

#[async_trait]
impl Database for MockDatabase {
    async fn create_user(
        &mut self,
        _patch: UserPatch,
    ) -> Result<User, Box<DatabaseError>> {
        unimplemented!("create_user is broken upstream; see Postgres::create_user")
    }

    async fn update_user(
        &mut self,
        _patch: UserPatch,
    ) -> Result<User, Box<DatabaseError>> {
        unimplemented!("update_user is broken upstream; see Postgres::update_user")
    }

    async fn read_user(&self, uuid: Uuid) -> Result<User, Box<DatabaseError>> {
        self.users
            .get(&uuid)
            .cloned()
            .ok_or_else(|| Box::new(DatabaseError::NotFound(uuid.to_string())))
    }

    async fn delete_user(&mut self, uuid: Uuid) -> Result<bool, Box<DatabaseError>> {
        Ok(self.users.remove(&uuid).is_some())
    }

    async fn register(
        &mut self,
        id: Uuid,
        username: String,
        first_name: String,
        last_name: String,
        email: String,
    ) -> Result<User, Box<DatabaseError>> {
        let now = chrono::Utc::now();
        let user = self.users.entry(id).or_insert_with(|| User {
            id,
            username: username.clone(),
            first_name: first_name.clone(),
            last_name: last_name.clone(),
            email: email.clone(),
            permissions: None,
            created_at: now,
            updated_at: now,
        });

        if user.username != username
            || user.first_name != first_name
            || user.last_name != last_name
            || user.email != email
        {
            user.username = username;
            user.first_name = first_name;
            user.last_name = last_name;
            user.email = email;
            user.updated_at = now;
        }

        Ok(user.clone())
    }

    async fn create_api_key(
        &mut self,
        owner: Uuid,
        name: String,
        hash: String,
        permissions: serde_json::Value,
    ) -> Result<ApiKey, Box<DatabaseError>> {
        if self.api_keys_by_hash.contains_key(&hash) {
            return Err(Box::new(DatabaseError::HashCollision));
        }

        let now = chrono::Utc::now();
        let key = ApiKey {
            id: Uuid::new_v4(),
            hash: hash.clone(),
            name,
            owner,
            permissions,
            created_at: now,
            updated_at: now,
        };
        self.api_keys_by_hash.insert(hash, key.clone());
        self.api_keys_by_id.insert(key.id, key.clone());
        Ok(key)
    }

    async fn read_api_key_by_id(&self, id: Uuid) -> Result<ApiKey, Box<DatabaseError>> {
        self.api_keys_by_id
            .get(&id)
            .cloned()
            .ok_or_else(|| Box::new(DatabaseError::NotFound(id.to_string())))
    }

    async fn read_api_key_by_hash(
        &self,
        hash: &str,
    ) -> Result<ApiKey, Box<DatabaseError>> {
        self.api_keys_by_hash
            .get(hash)
            .cloned()
            .ok_or_else(|| Box::new(DatabaseError::NotFound(hash.to_string())))
    }

    async fn delete_api_key(&mut self, id: Uuid) -> Result<bool, Box<DatabaseError>> {
        let Some(key) = self.api_keys_by_id.remove(&id) else {
            return Ok(false);
        };
        self.api_keys_by_hash.remove(&key.hash);
        Ok(true)
    }
}
