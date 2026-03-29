use std::sync::Arc;
use tokio::sync::RwLock;

use database::{
    Database,
    models::{User, UserPatch},
};

use crate::error::CoreError;

pub async fn create_user<U: Into<UserPatch>>(
    user: U,
    database: Arc<RwLock<dyn Database>>,
) -> Result<User, Box<CoreError>> {
    let patch: UserPatch = user.into();
    {
        let mut db = database.write().await;
        Ok(db.create_user(patch).await?)
    }
}
