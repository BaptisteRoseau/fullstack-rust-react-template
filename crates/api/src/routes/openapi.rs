use super::router::api_router;
use crate::endpoints::{
    api_key::endpoints::tag as api_key_tag, auth::endpoints::tag as auth_tag,
    ping::endpoints::tag as ping_tag, storage::endpoints::tag as storage_tag,
    user::endpoints::tag as user_tag,
};
use config::Config;
use utoipa::openapi::{
    InfoBuilder, OpenApi, Server,
    security::{ApiKey, ApiKeyValue, OpenIdConnect, SecurityRequirement, SecurityScheme},
};

/// Security scheme name for the JWT carried in the [`ACCESS_TOKEN_COOKIE`] cookie.
const JWT_COOKIE_SECURITY: &str = "OIDC";

/// Security scheme name for the API key carried in the `Authorization` header.
const API_KEY_SECURITY: &str = "API Key";

/// Metadata advertised in the generated OpenAPI document.
pub(super) fn api_info() -> utoipa::openapi::Info {
    InfoBuilder::new()
        .title(env!("CARGO_PKG_NAME"))
        .description(option_env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .build()
}

/// Builds the OpenAPI document describing the public API.
///
/// Requires no running service, so it can be serialized offline (e.g. to
/// generate the frontend API types).
pub fn openapi(config: &Config) -> OpenApi {
    let server = Server::new("/api");
    let (_, mut openapi) = api_router().split_for_parts();
    openapi.info = api_info();
    openapi.tags = Some(api_tags());
    openapi.servers = Some(vec![server]);
    add_security(&mut openapi, config);
    openapi
}

/// Declares the two ways a caller may authenticate, and requires either of them
/// (not both) on every operation.
///
/// - A JWT issued by the OIDC provider, sent as the `access_token` cookie set by
///   the auth BFF (`Cookie: access_token=<jwt>`).
/// - An API key sent bare in the `Authorization` header (`Authorization: <api_key>`).
///   The extractor also tolerates an optional `Bearer ` prefix on this header.
fn add_security(openapi: &mut OpenApi, config: &Config) {
    let components = openapi.components.get_or_insert_with(Default::default);
    components.add_security_scheme(
        JWT_COOKIE_SECURITY,
        SecurityScheme::OpenIdConnect(OpenIdConnect::with_description(
            config.authenticator.issuer_url.clone(),
            "OIDC authentication provider.".to_string(),
        )),
    );
    components.add_security_scheme(
        API_KEY_SECURITY,
        SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::with_description(
            "Authorization",
            "API key sent in the Authorization header.",
        ))),
    );

    openapi.security = Some(vec![
        SecurityRequirement::new(JWT_COOKIE_SECURITY, Vec::<String>::new()),
        SecurityRequirement::new(API_KEY_SECURITY, Vec::<String>::new()),
    ]);
}

/// Swagger categories, in display order. Each entry's name and description live
/// next to the endpoints it groups (see `endpoints::macros::declare_tag`).
fn api_tags() -> Vec<utoipa::openapi::Tag> {
    vec![
        auth_tag(),
        user_tag(),
        storage_tag(),
        api_key_tag(),
        ping_tag(),
    ]
}
