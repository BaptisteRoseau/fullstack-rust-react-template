//! HTTP request and response types for this endpoint group.
//!
//! Naming: <Method><Resource><"Request" | "Response" | "Params">.
//! Every doc comment here is published in the OpenAPI document, so write it for
//! the API consumer.

use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToResponse, ToSchema};
use uuid::Uuid;

/// A widget.
#[derive(Debug, Serialize, ToSchema, ToResponse)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetWidgetResponse {
    pub id: Uuid,
    pub name: String,
}

/// The widget to create.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PostWidgetRequest {
    pub name: String,
}

/// Filters applied when listing widgets.
#[derive(Debug, Deserialize, ToSchema, IntoParams)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetWidgetParams {
    pub page: Option<u32>,
}

// Convert from the app_core model here, so the handler stays free of mapping code.
//
// impl From<models::Widget> for GetWidgetResponse {
//     fn from(widget: models::Widget) -> Self { ... }
// }
