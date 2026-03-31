#[allow(dead_code)]
mod object_permissions;
mod permissions;
mod role;
mod scope;

pub use role::Role;
pub use permissions::Permissions;
pub use object_permissions::{ObjectPermissions, UserPermissions};
pub use scope::{PermissionScope, MixedPermissionScope};

// TODO: Unit tests, prefabs && documentation of the crate
// Rename to make more sense on what are restriction scope etc..
