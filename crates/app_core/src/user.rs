use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use database::{
    Database,
    error::DatabaseError,
    models::{User, UserPatch},
};

use crate::error::CoreError;

/// Registers a user from OIDC identity claims (`sub`, `preferred_username`,
/// `given_name`, `family_name`, `email`): creates the row on first login,
/// otherwise re-syncs the identity claims the provider owns.
///
/// The display name is only taken from the provider on creation; afterwards it
/// belongs to the user and is changed through [`update_profile`].
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

/// Reads the locally stored profile, or `None` when the user never logged in.
pub async fn read_profile(
    db: &dyn Database,
    id: Uuid,
) -> Result<Option<User>, CoreError> {
    match db.read_user(id).await {
        Ok(user) => Ok(Some(user)),
        Err(e) if matches!(*e, DatabaseError::NotFound(_)) => Ok(None),
        Err(e) => Err(CoreError::DatabaseError(e)),
    }
}

/// Changes the display name the user owns, leaving the provider's claims alone.
pub async fn update_profile(
    db: &mut dyn Database,
    id: Uuid,
    first_name: String,
    last_name: String,
) -> Result<User, CoreError> {
    let patch = User::build_patch(id)
        .set_first_name(first_name)
        .set_last_name(last_name);

    db.update_user(patch).await.map_err(|e| match *e {
        DatabaseError::NotFound(missing) => CoreError::NotFound(missing),
        other => CoreError::DatabaseError(Box::new(other)),
    })
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
