use serde::{Deserialize, Serialize};

/// Permissions of the application
#[derive(Debug, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub enum Permissions {
    UploadFile
}
