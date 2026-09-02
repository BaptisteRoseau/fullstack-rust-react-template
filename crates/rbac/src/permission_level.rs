use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// How much a user is allowed to do with one shared resource.
///
/// The variants are ordered, so "has at least this level" is a plain
/// comparison: `granted >= PermissionLevel::Editor`.
///
/// - [`Viewer`](Self::Viewer): list, download and preview.
/// - [`Editor`](Self::Editor): everything a viewer can do, plus uploading,
///   renaming and creating children.
/// - [`Manager`](Self::Manager): everything an editor can do, plus sharing,
///   revoking, moving and deleting.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "lowercase")]
pub enum PermissionLevel {
    Viewer,
    Editor,
    Manager,
}

/// Returned when a stored or user-supplied string names no known level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownPermissionLevel(pub String);

impl fmt::Display for UnknownPermissionLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown permission level: {}", self.0)
    }
}

impl std::error::Error for UnknownPermissionLevel {}

impl PermissionLevel {
    /// The lowercase name stored in the database and accepted over the API.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Viewer => "viewer",
            Self::Editor => "editor",
            Self::Manager => "manager",
        }
    }
}

impl FromStr for PermissionLevel {
    type Err = UnknownPermissionLevel;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "viewer" => Ok(Self::Viewer),
            "editor" => Ok(Self::Editor),
            "manager" => Ok(Self::Manager),
            other => Err(UnknownPermissionLevel(other.to_string())),
        }
    }
}

impl fmt::Display for PermissionLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

test_utils::tests_file!("_tests/test_permission_level.rs");
