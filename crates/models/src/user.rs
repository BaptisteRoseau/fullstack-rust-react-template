use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: uuid::Uuid,
    last_name: String,
    first_name: String,
    email: String,
    created_at: DateTime<chrono::Utc>,
    updated_at: DateTime<chrono::Utc>,
}
