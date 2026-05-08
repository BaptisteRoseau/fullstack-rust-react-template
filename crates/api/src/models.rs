use authenticator::UserToken as AuthUserToken;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

pub struct UserToken {
    pub id: Uuid,
    pub groups: HashSet<Uuid>,
    pub roles: HashSet<Uuid>,
}

impl From<AuthUserToken> for UserToken {
    fn from(value: AuthUserToken) -> Self {
        Self {
            id: value.id,
            groups: HashSet::new(),
            roles: HashSet::new(),
        }
    }
}

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
