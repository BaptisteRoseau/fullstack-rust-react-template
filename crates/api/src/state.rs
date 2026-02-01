use config::Config;
use crate::{
    database::{database::Database, postgres::PostgresDatabase},
};

use axum::extract::FromRef;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::error::ApiError;

// Notes:
// dyn trait are not supported for async functions.
// Don't try to use them yet, there is already a lot of
// time wasted to implement it.
//
// Try to use use an app state trait to encapsulate database logic.
// See: https://tulipemoutarde.be/posts/2023-08-20-depencency-injection-rust-axum/
// git stash show stash@{0} (WIP on main: 1db5b1e Add TODO)
//
// Do NOT use generics or dyn for AppState yet, find another way.

/// Application state containing all the components of the application
/// such as the database, the configuration or the authenticator.
///
/// All the mutable attributes should contain an Arc<RwLock<_>> to ensure
/// synchronization across the application.
///
/// All the immutable attributes should contain an Arc<_> to avoid
/// unnecessary data duplication (copy/cloning).
///
/// The mutable and immutable sub-states require Arc<RwLock<_>> and
/// Arc<_> to be accessed directly through Axum state. For example:
///
/// ```rs
/// use tokio::sync::RwLock;
/// use std::sync::Arc;
///
///pub(crate) async fn update_user(
///    State(state): State<AppState>, // Contains everything
///    State(database): State<Arc<RwLock<PostgresDatabase>>>, // Mutable -> Arc<RwLock<_>>
///    State(config): State<Arc<Config>>, // Immutable -> Arc<_>
///    State(authenticator): State<Arc<Authenticator>>, // Immutable -> Arc<_>
///) -> Result<String, String> {
///    ...
///}
/// ```
#[derive(Clone)]
pub(crate) struct AppState {
    pub core: Arc<RwLock<PostgresDatabase>>,
    pub config: Arc<Config>,
}

impl AppState {
    pub fn try_new(
        config: &Config,
        database: PostgresDatabase,
    ) -> Result<Self, ApiError> {
        Ok(Self {
            core: Arc::new(RwLock::new(database)),
            config: Arc::new(config.clone()),
        })
    }

    pub async fn close(&mut self) -> Result<(), ApiError> {
        self.core.write().await.close().await?;
        Ok(())
    }
}

impl FromRef<AppState> for Arc<Config> {
    fn from_ref(app_state: &AppState) -> Arc<Config> {
        app_state.config.clone()
    }
}

impl FromRef<AppState> for Arc<RwLock<PostgresDatabase>> {
    fn from_ref(app_state: &AppState) -> Arc<RwLock<PostgresDatabase>> {
        app_state.core.clone()
    }
}
