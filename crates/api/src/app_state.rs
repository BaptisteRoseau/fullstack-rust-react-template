use std::sync::Arc;

use authenticator::Authenticator;
use axum::extract::FromRef;
use cache::Cache;
use database::Database;
use storage::Storage;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub database: Arc<RwLock<dyn Database>>,
    pub storage: Arc<RwLock<dyn Storage>>,
    pub cache: Arc<RwLock<dyn Cache>>,
    pub authenticator: Arc<RwLock<dyn Authenticator>>,
}

impl AppState {
    pub fn new(
        database: Arc<RwLock<dyn Database>>,
        storage: Arc<RwLock<dyn Storage>>,
        cache: Arc<RwLock<dyn Cache>>,
        authenticator: Arc<RwLock<dyn Authenticator>>,
    ) -> Self {
        Self {
            database,
            storage,
            cache,
            authenticator,
        }
    }
}

impl FromRef<AppState> for Arc<RwLock<dyn Storage>> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.storage.clone()
    }
}

impl FromRef<AppState> for Arc<RwLock<dyn Database>> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.database.clone()
    }
}

impl FromRef<AppState> for Arc<RwLock<dyn Cache>> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.cache.clone()
    }
}

impl FromRef<AppState> for Arc<RwLock<dyn Authenticator>> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.authenticator.clone()
    }
}
