use authenticator::UserInfo;
use database::models::User;
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

/// The authenticated user's profile: identity from the OIDC provider, display
/// name from the locally stored row.
#[derive(Debug, Serialize, ToSchema, ToResponse)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetMeResponse {
    /// The OIDC subject the provider identifies the user by.
    pub id: String,
    /// Given name, owned by the user and changed through `PATCH /auth/me`.
    pub first_name: String,
    /// Family name, owned by the user and changed through `PATCH /auth/me`.
    pub last_name: String,
    /// Email address, owned by the identity provider.
    pub email: String,
    /// Application role. Not modelled yet: always `USER`.
    pub role: String,
    /// Team the user belongs to. Not modelled yet: always empty.
    pub team_id: String,
    /// First login, as a Unix timestamp in milliseconds. `0` until the user is
    /// registered locally.
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
            created_at: 0,
        }
    }

    /// Overlays the fields the local row owns onto the provider's claims.
    pub(crate) fn with_profile(mut self, user: &User) -> Self {
        self.first_name = user.first_name.clone();
        self.last_name = user.last_name.clone();
        self.created_at = user.created_at.timestamp_millis();
        self
    }
}

/// The profile fields a user may change about themselves.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PatchMeRequest {
    /// The new given name.
    pub first_name: String,
    /// The new family name.
    pub last_name: String,
}
