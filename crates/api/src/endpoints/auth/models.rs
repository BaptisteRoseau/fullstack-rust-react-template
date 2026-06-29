use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::{IntoParams, ToResponse, ToSchema};

/// Query parameters for starting the login or registration flow.
#[derive(Debug, Deserialize, ToSchema, IntoParams)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetLoginParams {
    /// Which Keycloak page to land on: "login" (default) or "register".
    pub screen: Option<String>,
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
    pub(crate) fn from_userinfo(info: &Value) -> Self {
        let claim = |key: &str| {
            info.get(key)
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string()
        };

        Self {
            id: claim("sub"),
            first_name: claim("given_name"),
            last_name: claim("family_name"),
            email: claim("email"),
            role: "USER".to_string(),
            team_id: String::new(),
            bio: String::new(),
            created_at: 0,
        }
    }
}
