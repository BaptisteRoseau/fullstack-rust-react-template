use crate::database::{DatabaseApiKey, DatabaseUser};
use async_trait::async_trait;

/// Defines the Database trait interface.
#[async_trait]
pub trait Database: Send + Sync + DatabaseApiKey + DatabaseUser {}

impl<T: Send + Sync + DatabaseApiKey + DatabaseUser> Database for T {}
