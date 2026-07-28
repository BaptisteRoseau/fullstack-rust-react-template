use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use database::{
    Database,
    models::{User, UserPatch},
};

use crate::error::CoreError;

/// Registers a user from OIDC identity claims (`sub`, `preferred_username`,
/// `given_name`, `family_name`, `email`): creates the row on first login,
/// otherwise syncs it if the identity provider's claims drifted.
pub async fn register(
    db: &mut dyn Database,
    id: Uuid,
    username: String,
    first_name: String,
    last_name: String,
    email: String,
) -> Result<User, CoreError> {
    Ok(db
        .register(id, username, first_name, last_name, email)
        .await?)
}

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

pub async fn update_user<U: Into<UserPatch>>(
    user: U,
    database: Arc<RwLock<dyn Database>>,
) -> Result<User, Box<CoreError>> {
    let patch: UserPatch = user.into();
    {
        let mut db = database.write().await;
        Ok(db.create_user(patch).await?)
    }
}

pub async fn get_user<U: Into<UserPatch>>(
    user: U,
    database: Arc<RwLock<dyn Database>>,
) -> Result<User, Box<CoreError>> {
    let patch: UserPatch = user.into();
    {
        let mut db = database.write().await;
        Ok(db.create_user(patch).await?)
    }
}

pub async fn delete_user<U: Into<UserPatch>>(
    user: U,
    database: Arc<RwLock<dyn Database>>,
) -> Result<User, Box<CoreError>> {
    let patch: UserPatch = user.into();
    {
        let mut db = database.write().await;
        Ok(db.create_user(patch).await?)
    }
}
