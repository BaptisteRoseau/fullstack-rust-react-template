use std::collections::HashSet;

use authenticator::Caller;
use rbac::{AccessControl, Scope};
use uuid::Uuid;

use crate::error::CoreError;

/// Refuses the caller unless they own the object, answering as if it did not exist.
pub(crate) fn ensure_owner(
    caller: &Caller,
    owner: Uuid,
    object_id: Uuid,
) -> Result<(), CoreError> {
    let access = AccessControl {
        required_permissions: None,
        scope: Scope::Users(HashSet::from([owner])),
    };
    if access.has_access(&caller.permissions()) {
        return Ok(());
    }
    Err(CoreError::NotFound(object_id.to_string()))
}
