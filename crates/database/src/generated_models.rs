#[derive(Debug, Clone, sqlx::FromRow, database_crud_derive::Crud)]
pub struct ApiKey {
    pub id: uuid::Uuid,
    pub hash: String,
    pub name: String,
    pub owner: uuid::Uuid,
    pub permissions: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow, database_crud_derive::Crud)]
pub struct User {
    pub id: uuid::Uuid,
    pub last_name: String,
    pub first_name: String,
    pub email: String,
    pub permissions: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
