// use std::sync::{Arc, Mutex};

// use async_trait::async_trait;
// use database::{
//     Database,
//     error::DatabaseError,
//     models::{ApiKey as DbApiKey, User, UserPatch},
//     testing::MockDatabase as SharedMockDatabase,
// };
// use uuid::Uuid;

// use super::create_api_key;

// /// Wraps the shared in-memory [`SharedMockDatabase`] but rigs
// /// `create_api_key` to fail with a forced hash collision on the first
// /// call: the real hash is randomly generated per attempt, so a generic
// /// map-backed mock can't reproduce a collision deterministically. This
// /// exercises `create_api_key`'s retry loop specifically, not storage.
// struct MockDatabase {
//     inner: SharedMockDatabase,
//     calls: Arc<Mutex<u32>>,
// }

// impl MockDatabase {
//     fn new() -> Self {
//         Self {
//             inner: SharedMockDatabase::default(),
//             calls: Arc::new(Mutex::new(0)),
//         }
//     }
// }

// #[async_trait]
// impl Database for MockDatabase {
//     async fn create_user(
//         &mut self,
//         patch: UserPatch,
//     ) -> Result<User, Box<DatabaseError>> {
//         self.inner.create_user(patch).await
//     }
//     async fn update_user(
//         &mut self,
//         patch: UserPatch,
//     ) -> Result<User, Box<DatabaseError>> {
//         self.inner.update_user(patch).await
//     }
//     async fn read_user(&self, uuid: Uuid) -> Result<User, Box<DatabaseError>> {
//         self.inner.read_user(uuid).await
//     }
//     async fn delete_user(&mut self, uuid: Uuid) -> Result<bool, Box<DatabaseError>> {
//         self.inner.delete_user(uuid).await
//     }
//     async fn register(
//         &mut self,
//         id: Uuid,
//         username: String,
//         first_name: String,
//         last_name: String,
//         email: String,
//     ) -> Result<User, Box<DatabaseError>> {
//         self.inner
//             .register(id, username, first_name, last_name, email)
//             .await
//     }

//     async fn read_api_key_by_id(
//         &self,
//         id: Uuid,
//     ) -> Result<DbApiKey, Box<DatabaseError>> {
//         self.inner.read_api_key_by_id(id).await
//     }
//     async fn read_api_key_by_hash(
//         &self,
//         hash: &str,
//     ) -> Result<DbApiKey, Box<DatabaseError>> {
//         self.inner.read_api_key_by_hash(hash).await
//     }
//     async fn delete_api_key(&mut self, id: Uuid) -> Result<bool, Box<DatabaseError>> {
//         self.inner.delete_api_key(id).await
//     }

//     async fn create_api_key(
//         &mut self,
//         _owner: Uuid,
//         _name: String,
//         _hash: String,
//         _permissions: serde_json::Value,
//     ) -> Result<DbApiKey, Box<DatabaseError>> {
//         let mut calls = self.calls.lock().unwrap();
//         *calls += 1;
//         let call_number = *calls;
//         drop(calls);

//         if call_number == 1 {
//             // Simulate hash collision on first call
//             Err(Box::new(DatabaseError::HashCollision))
//         } else {
//             // Sentinel error to exit the retry loop cleanly
//             Err(Box::new(DatabaseError::NotFound("test-sentinel".into())))
//         }
//     }
// }

// #[tokio::test]
// async fn test_hash_collision_retry() {
//     let mut mock = MockDatabase::new();
//     let calls = mock.calls.clone();

//     let result =
//         create_api_key(&mut mock, Uuid::new_v4(), "my key".into(), vec![]).await;

//     let total_calls = *calls.lock().unwrap();
//     assert_eq!(
//         total_calls, 2,
//         "expected 2 DB calls (1 collision + 1 sentinel), got {total_calls}"
//     );
//     assert!(result.is_err(), "expected error from sentinel, got Ok");
//     // Should NOT be a hash collision error (that was retried)
//     if let Err(crate::error::CoreError::DatabaseError(e)) = &result {
//         assert!(
//             !e.is_hash_collision(),
//             "error should not be a hash collision after retry, got: {e:?}"
//         );
//     }
// }
