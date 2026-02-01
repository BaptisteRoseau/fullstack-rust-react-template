#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    id: uuid::Uuid,
    last_name: String,
    first_name: String,
    email: String,
    address: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}
