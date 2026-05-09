use crate::error::CoreError;

pub fn api_key_from_db(db_key: database::models::ApiKey) -> Result<models::ApiKey, CoreError> {
    let permissions: Vec<rbac::Permissions> =
        serde_json::from_str(db_key.permissions().as_str())?;
    Ok(models::ApiKey {
        id: db_key.id(),
        name: db_key.name().to_string(),
        owner: db_key.owner(),
        permissions,
        created_at: db_key.created_at(),
    })
}
