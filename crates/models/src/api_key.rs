use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: uuid::Uuid,
    pub name: String,
    pub owner: uuid::Uuid,
    pub permissions: Vec<rbac::Permissions>,
    pub created_at: DateTime<chrono::Utc>,
}
