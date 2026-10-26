use serde::{Deserialize, Serialize};

/// Permissions of the application
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub enum Permissions {
    UploadFile,
}
