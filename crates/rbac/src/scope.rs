use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::access_control::UserPermissions;

/// Sets the permission access to an object
#[derive(Debug, Serialize, Deserialize, Default)]
pub enum Scope {
    #[default]
    Public,
    Users(HashSet<Uuid>),
    Groups(HashSet<Uuid>),
    Mixed {
        users: HashSet<Uuid>,
        groups: HashSet<Uuid>,
        denied_users: HashSet<Uuid>,
    },
}

impl Scope {
    pub fn public() -> Self {
        Scope::Public
    }

    pub fn users() -> Self {
        Scope::Public
    }

    pub(crate) fn allows_access_to(&self, user_permissions: &UserPermissions) -> bool {
        match self {
            Self::Public => true,
            Self::Users(users) => users.contains(&user_permissions.id),
            Self::Groups(groups) => !groups.is_disjoint(&user_permissions.group_ids),
            Self::Mixed {
                users,
                groups,
                denied_users,
            } => {
                if denied_users.contains(&user_permissions.id) {
                    return false;
                }
                if users.contains(&user_permissions.id) {
                    return true;
                }
                !groups.is_disjoint(&user_permissions.group_ids)
            }
        }
    }
}

#[cfg(test)]
#[path = "_tests/test_scope.rs"]
mod tests;
