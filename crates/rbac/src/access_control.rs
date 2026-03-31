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

#[cfg(test)]
mod test {
    use super::*;

    fn make_user(permissions: HashSet<Permissions>) -> UserPermissions {
        UserPermissions::new(Uuid::new_v4(), HashSet::new(), permissions)
    }

    #[test]
    fn default_is_public_no_permissions() {
        let obj = ObjectPermissions::default();

        assert!(
            obj.required_permissions.is_none(),
            "Default should have no required permissions"
        );
        assert!(
            matches!(obj.scope, PermissionScope::Public),
            "Default scope should be Public, got {:?}",
            obj.scope
        );
    }

    #[test]
    fn default_grants_access_to_any_user() {
        let obj = ObjectPermissions::default();
        let user = make_user(HashSet::new());

        assert!(
            obj.has_access(&user),
            "Default ObjectPermissions should grant access to any user"
        );
    }

    #[test]
    fn set_scope_changes_scope() {
        let mut obj = ObjectPermissions::default();
        let user = make_user(HashSet::new());

        let mut allowed_users = HashSet::new();
        allowed_users.insert(Uuid::new_v4());
        obj.set_scope(PermissionScope::Users(allowed_users));

        assert!(
            !obj.has_access(&user),
            "User not in scope should be denied access"
        );
    }

    #[test]
    fn set_permissions_with_empty_set_clears() {
        let mut obj = ObjectPermissions::default();
        let mut perms = HashSet::new();
        perms.insert(Permissions::UploadFile);
        obj.set_permissions(perms);

        assert!(
            obj.required_permissions.is_some(),
            "Should have required permissions after setting non-empty set"
        );

        obj.set_permissions(HashSet::new());

        assert!(
            obj.required_permissions.is_none(),
            "Setting empty permissions should clear required_permissions to None"
        );
    }

    #[test]
    fn access_denied_without_required_permission() {
        let mut obj = ObjectPermissions::default();
        let mut required = HashSet::new();
        required.insert(Permissions::UploadFile);
        obj.set_permissions(required);

        let user = make_user(HashSet::new());

        assert!(
            !obj.has_access(&user),
            "User without UploadFile permission should be denied"
        );
    }

    #[test]
    fn access_granted_with_required_permission() {
        let mut obj = ObjectPermissions::default();
        let mut required = HashSet::new();
        required.insert(Permissions::UploadFile);
        obj.set_permissions(required);

        let mut user_perms = HashSet::new();
        user_perms.insert(Permissions::UploadFile);
        let user = make_user(user_perms);

        assert!(
            obj.has_access(&user),
            "User with UploadFile permission should be granted access"
        );
    }

    #[test]
    fn access_denied_with_permission_but_wrong_scope() {
        let mut obj = ObjectPermissions::default();

        let mut required = HashSet::new();
        required.insert(Permissions::UploadFile);
        obj.set_permissions(required);

        let mut allowed_users = HashSet::new();
        allowed_users.insert(Uuid::new_v4());
        obj.set_scope(PermissionScope::Users(allowed_users));

        let mut user_perms = HashSet::new();
        user_perms.insert(Permissions::UploadFile);
        let user = make_user(user_perms);

        assert!(
            !obj.has_access(&user),
            "User with correct permission but outside scope should be denied"
        );
    }

    #[test]
    fn access_denied_with_scope_but_missing_permission() {
        let mut obj = ObjectPermissions::default();
        let user = make_user(HashSet::new());

        let mut allowed_users = HashSet::new();
        allowed_users.insert(user.id);
        obj.set_scope(PermissionScope::Users(allowed_users));

        let mut required = HashSet::new();
        required.insert(Permissions::UploadFile);
        obj.set_permissions(required);

        assert!(
            !obj.has_access(&user),
            "User in scope but missing required permission should be denied"
        );
    }

    #[test]
    fn access_granted_with_permission_and_scope() {
        let mut obj = ObjectPermissions::default();

        let mut user_perms = HashSet::new();
        user_perms.insert(Permissions::UploadFile);
        let user = make_user(user_perms);

        let mut allowed_users = HashSet::new();
        allowed_users.insert(user.id);
        obj.set_scope(PermissionScope::Users(allowed_users));

        let mut required = HashSet::new();
        required.insert(Permissions::UploadFile);
        obj.set_permissions(required);

        assert!(
            obj.has_access(&user),
            "User with correct permission and in scope should be granted access"
        );
    }

    #[test]
    fn set_methods_return_mutable_self() {
        let mut obj = ObjectPermissions::default();
        let mut users = HashSet::new();
        users.insert(Uuid::new_v4());

        let mut perms = HashSet::new();
        perms.insert(Permissions::UploadFile);

        // Test method chaining
        obj.set_scope(PermissionScope::Users(users))
            .set_permissions(perms);

        assert!(
            obj.required_permissions.is_some(),
            "Chained set_permissions should work"
        );
        assert!(
            matches!(obj.scope, PermissionScope::Users(_)),
            "Chained set_scope should work, got {:?}",
            obj.scope
        );
    }
}
