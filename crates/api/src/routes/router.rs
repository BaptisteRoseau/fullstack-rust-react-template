use super::middlewares::with_middlewares;
use super::openapi::{api_info, openapi};
use crate::{
    app_state::AppState,
    endpoints::{
        api_key::endpoints::{
            __path_create_api_key, __path_delete_api_key, __path_get_api_key,
            __path_list_api_keys, create_api_key, delete_api_key, get_api_key,
            list_api_keys,
        },
        auth::endpoints::{
            __path_callback, __path_login, __path_logout, __path_me, __path_refresh,
            __path_register, __path_update_me, callback, login, logout, me, refresh,
            register, update_me,
        },
        files::endpoints::{
            __path_create_directory, __path_delete_directory, __path_delete_file,
            __path_download_file, __path_download_thumbnail, __path_get_file,
            __path_grant_directory_permission, __path_grant_file_permission,
            __path_list_directory_permissions, __path_list_entries,
            __path_list_file_permissions, __path_revoke_directory_permission,
            __path_revoke_file_permission, __path_update_directory, __path_update_file,
            __path_upload_file, create_directory, delete_directory, delete_file,
            download_file, download_thumbnail, get_file, grant_directory_permission,
            grant_file_permission, list_directory_permissions, list_entries,
            list_file_permissions, revoke_directory_permission, revoke_file_permission,
            update_directory, update_file, upload_file,
        },
        ping::endpoints::{__path_ping, ping},
        storage::endpoints::{
            __path_delete_stored_file, __path_download, __path_upload,
            delete_stored_file, download, upload,
        },
        user::endpoints::{__path_get_user, get_user},
    },
};
use axum::{Router, routing::get};
use axum_prometheus::metrics_exporter_prometheus::PrometheusHandle;
use config::Config;
use std::future::ready;
use utoipa::openapi::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

// Bookmark this: https://docs.rs/axum/latest/axum/routing/struct.Router.html

/// Builds the [`OpenApiRouter`] holding every public API route.
///
/// This only assembles the route definitions and their generated schemas, so it
/// can be called without any running service (database, cache, storage, ...).
/// It is the single source of truth shared by [`public_routes`] and [`super::openapi::openapi`].
pub(super) fn api_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::<AppState>::new()
        .routes(routes!(ping))
        .routes(routes!(get_user))
        // Object storage
        .routes(routes!(upload))
        .routes(routes!(download))
        .routes(routes!(delete_stored_file))
        // Files: the encrypted, shareable file tree
        .routes(routes!(list_entries))
        .routes(routes!(upload_file))
        .routes(routes!(create_directory))
        .routes(routes!(update_directory, delete_directory))
        .routes(routes!(list_directory_permissions))
        .routes(routes!(
            grant_directory_permission,
            revoke_directory_permission,
        ))
        .routes(routes!(get_file, update_file, delete_file))
        .routes(routes!(download_file))
        .routes(routes!(download_thumbnail))
        .routes(routes!(list_file_permissions))
        .routes(routes!(grant_file_permission, revoke_file_permission))
        // API Key
        .routes(routes!(create_api_key, list_api_keys))
        .routes(routes!(get_api_key))
        .routes(routes!(delete_api_key))
        // Authentication
        .routes(routes!(login))
        .routes(routes!(register))
        .routes(routes!(callback))
        .routes(routes!(refresh))
        .routes(routes!(logout))
        .routes(routes!(me, update_me))
}

/// Public routes that qre exposed to the world
pub fn public_routes(config: &Config, state: AppState) -> Router {
    let (api_routes, _) = api_router().split_for_parts();
    // The whole API (typed endpoints + auth BFF) lives under `/api`, matching the
    // frontend's `VITE_APP_API_URL`. Swagger keeps its own absolute path at the root.
    let routes = Router::new()
        .nest("/api", api_routes)
        .merge(swagger(config, openapi(config)));

    with_middlewares(routes, config).with_state(state)
}

/// Swagger UI and OpenAPI routes layer.
fn swagger(config: &Config, openapi: OpenApi) -> SwaggerUi {
    let swagger_config = config.swagger.as_ref().unwrap().clone();
    let swagger_ui_path = swagger_config.swagger_ui_path;
    let openapi_path = swagger_config.openapi_path;
    let mut openapi = openapi;
    openapi.info = api_info();
    SwaggerUi::new(swagger_ui_path).url(openapi_path, openapi)
}

/// Metrics routes that are exposed to Prometheus
pub fn try_metrics_routes(
    path: &str,
    metric_handle: PrometheusHandle,
) -> Result<Router, anyhow::Error> {
    Ok(Router::new().route(path, get(move || ready(metric_handle.render()))))
}
