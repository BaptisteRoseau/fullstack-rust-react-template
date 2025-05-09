use uuid::Uuid;

pub(crate) struct StoredLoginInfo{
    pub password_hash: String,
    pub salt: String,
    pub id: Uuid,
    pub name: String,
    pub email: String
}