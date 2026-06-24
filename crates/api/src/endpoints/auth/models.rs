use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToResponse, ToSchema};
use uuid::Uuid;

/// Role assigned to an authenticated user.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum UserRole {
    Admin,
    User,
}

/// Information about the currently authenticated user.
#[derive(Debug, Serialize, ToSchema, ToResponse)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetMeResponse {
    pub id: Uuid,
    pub email: Option<String>,
    pub role: UserRole,
}

/// Optional redirect destination after login.
#[derive(Debug, Deserialize, ToSchema, IntoParams)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LoginParams {
    /// Frontend URL to redirect to after a successful login (overrides server default).
    pub redirect: Option<String>,
}

/// Token response from Keycloak token endpoint.
#[derive(Debug, Deserialize)]
pub(crate) struct KeycloakTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    pub refresh_expires_in: u64,
}
