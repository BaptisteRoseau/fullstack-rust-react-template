use std::collections::HashSet;

use authenticator::UserToken as AuthUserToken;
use rbac::UserPermissions;
use uuid::Uuid;

pub struct UserToken {
    pub id: Uuid,
    pub _realm: String,
}

impl UserToken {
    /// The caller handed to `app_core`, which runs every access check.
    pub fn caller(&self) -> UserPermissions {
        caller_from_id(self.id)
    }
}

impl From<AuthUserToken> for UserToken {
    fn from(value: AuthUserToken) -> Self {
        Self {
            id: value.id,
            _realm: value.realm,
        }
    }
}

/// A caller with no group and no granted permission, the only kind a token carries today.
pub fn caller_from_id(id: Uuid) -> UserPermissions {
    UserPermissions::new(id, HashSet::new(), HashSet::new())
}
