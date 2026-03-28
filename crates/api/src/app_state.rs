use std::sync::Arc;

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
}

impl AppState {
    pub fn new(
        database: Arc<RwLock<dyn Database>>,
        storage: Arc<RwLock<dyn Storage>>,
        cache: Arc<RwLock<dyn Cache>>,
    ) -> Self {
        Self {
            database,
            storage,
            cache,
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
