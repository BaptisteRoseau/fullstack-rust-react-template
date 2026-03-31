use crate::{permissions::Permissions, scope::Scope};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

/// Defines the scope of what a user has access to.
pub struct UserPermissions {
    pub(crate) id: Uuid,
    pub(crate) group_ids: HashSet<Uuid>,
    pub(crate) permissions: HashSet<Permissions>,
}

impl UserPermissions {
    pub fn new(
        id: Uuid,
        group_ids: HashSet<Uuid>,
        permissions: HashSet<Permissions>,
    ) -> Self {
        Self {
            id,
            group_ids,
            permissions,
        }
    }
}

/// Permissions of an object
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessControl {
    pub required_permissions: Option<HashSet<Permissions>>,
    pub scope: Scope,
}

impl Default for AccessControl {
    /// Default permission are permissive: no required permission and scope is Public.
    fn default() -> Self {
        Self {
            required_permissions: None,
            scope: Scope::Public,
        }
    }
}

impl AccessControl {
    pub fn set_scope(&mut self, scope: Scope) -> &mut Self {
        self.scope = scope;
        self
    }

    pub fn set_permissions(&mut self, permissions: HashSet<Permissions>) -> &mut Self {
        if permissions.is_empty() {
            self.required_permissions = None
        } else {
            self.required_permissions = Some(permissions)
        }
        self
    }

    /// Verify if the user has access to the object. Should match both required permissions
    /// if any AND the scope of access of the object.
    pub fn has_access(&self, user_permissions: &UserPermissions) -> bool {
        if let Some(permissions) = &self.required_permissions
            && !user_permissions.permissions.is_superset(permissions)
        {
            return false;
        }
        self.scope.allows_access_to(user_permissions)
    }
}
