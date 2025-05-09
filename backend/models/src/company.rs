use super::profile::ProfileInfo;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct CompanyContent {
    pub name: String,
}

impl CompanyContent {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct Company {
    pub id: Uuid,
    pub owner: Uuid,
    #[serde(flatten)]
    pub content: CompanyContent,
}
// TODO: Stop using the "content" just to satisfy axum but prefer leaving Axum on it own layer
// with its own structures that will implement Into<Company>.
// We can do something similar with the database layer

impl Company {
    pub fn new(id: Uuid, owner: Uuid, content: CompanyContent) -> Self {
        Self { id, owner, content }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CompanyInfo {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub favorites_count: u32,
    pub author: ProfileInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CompanyInfoWrapper {
    pub company: CompanyInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompanyListInfo {
    pub companies: Vec<CompanyInfo>,
    pub companies_count: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CompanyCreateUpdateInfo {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompanyCreateUpdateInfoWrapper {
    pub company: CompanyCreateUpdateInfo,
}
