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

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}
