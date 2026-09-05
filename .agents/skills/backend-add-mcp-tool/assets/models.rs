//! Parameter and result types for this tool group.
//!
//! Naming: <Tool><"Params" | "Result">.
//! `JsonSchema` turns these into the tool's inputSchema, so every doc comment here
//! is read by the model deciding how to call the tool. Write it for that reader.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Arguments of the `get_widget` tool.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetWidgetParams {
    /// Identifier of the widget to read, as a UUID.
    pub widget_id: Uuid,
}

/// A widget, as returned by the `get_widget` tool.
///
/// Carries what the caller asked for and nothing more: an assistant sees every field.
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetWidgetResult {
    pub id: Uuid,
    pub name: String,
}
