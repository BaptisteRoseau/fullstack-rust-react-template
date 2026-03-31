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
        let scope = Scope::Public;
        assert!(scope.allows_access_to(&user_permissions()))
    }

    #[test]
    fn user_access_allowed() {
        let permissions = &user_permissions();

        let mut scope_uids = HashSet::new();
        scope_uids.insert(Uuid::new_v4());
        scope_uids.insert(permissions.id);
        let scope = Scope::Users(scope_uids);

        assert!(scope.allows_access_to(permissions))
    }

    #[test]
    fn user_access_denied() {
        let permissions = &user_permissions();

        let mut scope_uids = HashSet::new();
        scope_uids.insert(Uuid::new_v4());
        scope_uids.insert(Uuid::new_v4());
        let scope = Scope::Users(scope_uids);

        assert!(!scope.allows_access_to(permissions))
    }

    #[test]
    fn group_access_allowed() {
        let permissions = &user_permissions();

        let mut scope_gids = HashSet::new();
        scope_gids.insert(Uuid::new_v4());
        scope_gids.insert(*permissions.group_ids.iter().next().unwrap());
        let scope = Scope::Groups(scope_gids);

        assert!(scope.allows_access_to(permissions))
    }

    #[test]
    fn group_access_denied() {
        let permissions = &user_permissions();

        let mut scope_gids = HashSet::new();
        scope_gids.insert(Uuid::new_v4());
        scope_gids.insert(Uuid::new_v4());
        let scope = Scope::Groups(scope_gids);

        assert!(!scope.allows_access_to(permissions))
    }

    #[test]
    fn mixed_access_allowed_user_match() {
        let permissions = &user_permissions();

        let mut users = HashSet::new();
        users.insert(permissions.id);
        users.insert(Uuid::new_v4());

        let groups = HashSet::new();

        let scope = Scope::Mixed {
            users,
            groups,
            denied_users: HashSet::default(),
        };

        assert!(
            scope.allows_access_to(permissions),
            "Mixed scope should allow access when user id {:?} is in the users set",
            permissions.id
        );
    }

    #[test]
    fn mixed_access_allowed_group_match() {
        let permissions = &user_permissions();

        let users = HashSet::new();

        let mut groups = HashSet::new();
        groups.insert(*permissions.group_ids.iter().next().unwrap());
        groups.insert(Uuid::new_v4());

        let scope = Scope::Mixed {
            users,
            groups,
            denied_users: HashSet::default(),
        };

        assert!(
            scope.allows_access_to(permissions),
            "Mixed scope should allow access when a group id matches"
        );
    }

    #[test]
    fn mixed_access_denied() {
        let permissions = &user_permissions();

        let mut users = HashSet::new();
        users.insert(Uuid::new_v4());

        let mut groups = HashSet::new();
        groups.insert(Uuid::new_v4());

        let scope = Scope::Mixed {
            users,
            groups,
            denied_users: HashSet::default(),
        };

        assert!(
            !scope.allows_access_to(permissions),
            "Mixed scope should deny access when neither user id nor group ids match"
        );
    }

    #[test]
    fn mixed_specific_user_access_denied() {
        let permissions = &user_permissions();

        // User is in a group that is allowed to access the item...
        let mut groups = HashSet::new();
        groups.insert(Uuid::new_v4());

        // .. but has specifically been denied.
        let mut denied_users = HashSet::new();
        denied_users.insert(Uuid::new_v4());

        let scope = Scope::Mixed {
            users: HashSet::default(),
            groups,
            denied_users,
        };

        assert!(
            !scope.allows_access_to(permissions),
            "Mixed scope should deny access when neither user id nor group ids match"
        );
    }
}
