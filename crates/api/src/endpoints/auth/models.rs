use authenticator::UserInfo;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToResponse, ToSchema};

/// Query parameters for starting the login or registration flow.
#[derive(Debug, Deserialize, ToSchema, IntoParams)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetLoginParams {
    /// Same-origin path to return to after a successful login.
    pub redirect: Option<String>,
}

/// Query parameters Keycloak appends when redirecting to the OAuth callback.
#[derive(Debug, Deserialize, ToSchema, IntoParams)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetCallbackParams {
    /// Authorization code issued by Keycloak.
    pub code: Option<String>,
    /// CSRF state echoed back by Keycloak.
    pub state: Option<String>,
}

/// The authenticated user's profile, derived from Keycloak's userinfo claims.
#[derive(Debug, Serialize, ToSchema, ToResponse)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetMeResponse {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub role: String,
    pub team_id: String,
    pub bio: String,
    pub created_at: i64,
}

impl GetMeResponse {
    /// Maps OIDC userinfo claims onto the user profile shape.
    pub(crate) fn from_userinfo(info: &UserInfo) -> Self {
        Self {
            id: info.sub.to_string(),
            first_name: info.given_name.clone(),
            last_name: info.family_name.clone(),
            email: info.email.clone(),
            role: "USER".to_string(),
            team_id: String::new(),
            bio: String::new(),
            created_at: 0,
        }
    }
}
