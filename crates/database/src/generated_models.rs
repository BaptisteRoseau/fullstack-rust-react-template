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
    pub(crate) id: uuid::Uuid,
    pub(crate) hash: String,
    pub(crate) name: String,
    pub(crate) owner: uuid::Uuid,
    pub(crate) permissions: serde_json::Value,
    pub(crate) created_at: chrono::DateTime<chrono::Utc>,
    pub(crate) updated_at: chrono::DateTime<chrono::Utc>,
}
