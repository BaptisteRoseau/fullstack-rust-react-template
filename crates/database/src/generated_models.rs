#[derive(Debug, Clone, sqlx::FromRow, database_crud_derive::Crud)]
pub struct User {
    id: uuid::Uuid,
    last_name: String,
    first_name: String,
    email: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ApiKey {
    pub id: uuid::Uuid,
    pub hash: String,
    pub name: String,
    pub owner: uuid::Uuid,
    pub permissions: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
