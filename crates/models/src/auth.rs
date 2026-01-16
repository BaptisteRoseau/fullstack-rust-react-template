use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// TODO: Actually move each model in its corresponding crate

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct LoginInfo {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoginInfoWrapper {
    pub user: LoginInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct RegisterInfo {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RegisterInfoWrapper {
    pub user: RegisterInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct UserInfo {
    pub email: String,
    pub token: String,
    pub name: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct UserInfoWrapper {
    pub user: UserInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct UserUpdateInfo {
    pub email: String,
    pub username: String,
    pub password: Option<String>,
    pub image: String,
    pub bio: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserUpdateInfoWrapper {
    pub user: UserUpdateInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserRegistration {
    pub id: uuid::Uuid,
    pub credentials: UserCredentials,
    pub email: String,
    pub name: Option<String>,
    pub surname: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}
