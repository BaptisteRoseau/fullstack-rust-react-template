use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::object_permissions::UserPermissions;
/// An object that can be accessed either by a groups or by users.
#[derive(Debug, Serialize, Deserialize)]
pub struct MixedPermissionScope {
    pub users: HashSet<Uuid>,
    pub groups: HashSet<Uuid>,
}

impl MixedPermissionScope {
    pub fn new(users: HashSet<Uuid>, groups: HashSet<Uuid>) -> Self {
        Self { users, groups }
    }
}

/// Sets the permission access to an object
#[derive(Debug, Serialize, Deserialize)]
pub enum PermissionScope {
    Public,
    Users(HashSet<Uuid>),
    Groups(HashSet<Uuid>),
    Mixed(MixedPermissionScope),
}

impl PermissionScope {
    pub(crate) fn allows_access_to(&self, user_permissions: &UserPermissions) -> bool {
        match self {
            Self::Public => true,
            Self::Users(users) => users.contains(&user_permissions.id),
            Self::Groups(groups) => !groups.is_disjoint(&user_permissions.group_ids),
            Self::Mixed(mixed) => {
                if mixed.users.contains(&user_permissions.id) {
                    return true;
                }
                !mixed.groups.is_disjoint(&user_permissions.group_ids)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn user_permissions() -> UserPermissions {
        let id = Uuid::new_v4();
        let mut group_ids = HashSet::new();
        let permissions = HashSet::new();

        group_ids.insert(Uuid::new_v4());
        group_ids.insert(Uuid::new_v4());

        UserPermissions {
            id,
            group_ids,
            permissions,
        }
    }

    #[test]
    fn has_access_to_public_scope() {
        let scope = PermissionScope::Public;
        assert!(scope.allows_access_to(&user_permissions()))
    }

    #[test]
    fn user_access_allowed() {
        let permissions = &user_permissions();

        let mut scope_uids = HashSet::new();
        scope_uids.insert(Uuid::new_v4());
        scope_uids.insert(permissions.id);
        let scope = PermissionScope::Users(scope_uids);

        assert!(scope.allows_access_to(permissions))
    }

    #[test]
    fn user_access_denied() {
        let permissions = &user_permissions();

        let mut scope_uids = HashSet::new();
        scope_uids.insert(Uuid::new_v4());
        scope_uids.insert(Uuid::new_v4());
        let scope = PermissionScope::Users(scope_uids);

        assert!(!scope.allows_access_to(permissions))
    }

    #[test]
    fn group_access_allowed() {
        let permissions = &user_permissions();

        let mut scope_gids = HashSet::new();
        scope_gids.insert(Uuid::new_v4());
        scope_gids.insert(*permissions.group_ids.iter().next().unwrap());
        let scope = PermissionScope::Groups(scope_gids);

        assert!(scope.allows_access_to(permissions))
    }

    #[test]
    fn group_access_denied() {
        let permissions = &user_permissions();

        let mut scope_gids = HashSet::new();
        scope_gids.insert(Uuid::new_v4());
        scope_gids.insert(Uuid::new_v4());
        let scope = PermissionScope::Groups(scope_gids);

        assert!(!scope.allows_access_to(permissions))
    }

    #[test]
    #[ignore]
    fn mixed_access_allowed_user_match() {

        // assert!(scope.allows_access_to(permissions))
    }

    #[test]
    #[ignore]
    fn mixed_access_allowed_group_match() {

        // assert!(scope.allows_access_to(permissions))
    }

    #[test]
    #[ignore]
    fn mixed_access_denied() {

        // assert!(!scope.allows_access_to(permissions))
    }
}
