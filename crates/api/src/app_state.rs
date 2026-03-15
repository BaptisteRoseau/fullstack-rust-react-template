use std::sync::{Arc, RwLock};

use axum::extract::FromRef;
use database::Database;
use storage::Storage;

#[derive(Clone)]
pub struct AppState {
    pub database: Arc<RwLock<dyn Database>>,
    pub storage: Arc<RwLock<dyn Storage>>,
}

impl AppState {
    pub fn new(
        database: Arc<RwLock<dyn Database>>,
        storage: Arc<RwLock<dyn Storage>>,
    ) -> Self {
        Self { database, storage }
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
