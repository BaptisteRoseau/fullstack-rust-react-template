#[allow(dead_code)]
mod access_control;
mod permissions;
mod role;
mod scope;

pub use access_control::{AccessControl, UserPermissions};
pub use permissions::Permissions;
pub use role::Role;
pub use scope::Scope;

// TODO: Unit tests, prefabs && documentation of the crate
// Rename to make more sense on what are restriction scope etc..
