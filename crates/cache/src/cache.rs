use std::collections::HashMap;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait Cache: Send + Sync {
    async fn set<T: Serialize + Deserialize<'de>>(
        key: &'a dyn ToString,
        value: &'de T,
        timeout_s: Option<u32>,
    );
    async fn get<T: Deserialize>(key: &dyn ToString) -> T;
    async fn delete<T: Deserialize>(key: &dyn ToString);

    async fn set_many<T: Serialize + Deserialize>(
        mappings: HashMap<&dyn ToString, T>,
        timeout_s: Option<u32>,
    );

    async fn get_many<T: Deserialize>(key: &[&dyn ToString]) -> HashMap<String, T>;
    async fn delete_many<T: Deserialize>(keys: &[&dyn ToString]);
}
