use serde::{Deserialize, Serialize};

use crate::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInvoiceInfo {
    id: uuid::Uuid,
    user: User,
    adress: String,
    zip_code: u32,
    city: String,
    country: String,
    company_name: Option<String>,
    company_code: Option<String>,
}
