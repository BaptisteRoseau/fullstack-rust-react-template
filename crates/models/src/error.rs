//! Errors
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Fidbak api error info for Unprocessable Entity error
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ApiErrorInfo {
    pub id: String,
    pub error: String,
    pub errors: HashMap<String, Vec<String>>,
}
