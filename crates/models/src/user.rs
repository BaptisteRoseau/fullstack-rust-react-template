// TODO: Crate 'schemas' to avoid duplication for crates/frontend

use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

type Mail = String;

#[derive(Clone, Debug, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct UserContent {
    pub username: String,
    pub email: Mail,
    pub password: Option<String>,
}

impl UserContent {
    pub fn new(name: String, mail: Mail) -> Self {
        Self {
            username: name,
            email: mail,
            password: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct User {
    pub id: Uuid,
    #[serde(flatten)]
    pub content: UserContent,
}

impl User {
    pub fn new(id: Uuid, content: UserContent) -> Self {
        Self { id, content }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginInfo {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginInfoWrapper {
    pub user: LoginInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterInfo {
    pub surname: String,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterInfoWrapper {
    pub user: RegisterInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub email: String,
    pub token: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserInfoWrapper {
    pub user: UserInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdateInfo {
    pub email: String,
    pub username: String,
    pub password: Option<String>,
    pub image: String,
    pub bio: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdateInfoWrapper {
    pub user: UserUpdateInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct UserTokenResponse {
    pub token: String,
}
