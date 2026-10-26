use std::collections::HashSet;

use rbac::{AccessControl, Scope, UserPermissions};
use uuid::Uuid;

use crate::error::CoreError;

/// Refuses the caller unless they own the object, answering as if it did not exist.
pub(crate) fn ensure_owner(
    caller: &UserPermissions,
    owner: Uuid,
    object_id: Uuid,
) -> Result<(), CoreError> {
    let access = AccessControl {
        required_permissions: None,
        scope: Scope::Users(HashSet::from([owner])),
    };
    if access.has_access(caller) {
        return Ok(());
    }
    Err(CoreError::NotFound(object_id.to_string()))
}
