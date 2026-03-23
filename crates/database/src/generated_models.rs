use database_crud_derive::Crud;

#[derive(Debug, Clone, sqlx::FromRow, Crud)]
pub struct User {
    pub id: uuid::Uuid,
    pub last_name: String,
    pub first_name: String,
    pub email: String,
    pub address: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
