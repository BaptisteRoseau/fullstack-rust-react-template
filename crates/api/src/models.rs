use serde::{Deserialize, Serialize};

/// User information provided from the JWT
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    id: String,
    name: String,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name(), self.id())
    }
}

impl User {
    pub(super) fn new(id: &dyn ToString, name: &dyn ToString) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
        }
    }

    /// The String ID of a user if logged in, None instead.
    pub fn id(&self) -> String {
        self.id.clone()
    }

    /// The String user of a user if logged in, None instead.
    pub fn name(&self) -> String {
        self.name.clone()
    }
}
