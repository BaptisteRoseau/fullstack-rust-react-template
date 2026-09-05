#[allow(clippy::module_inception)]
mod error;
mod response;

pub(crate) use error::McpError;
pub(crate) use response::{into_tool_result, structured};
