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
// serialize/deserialize tests in integration to ensure smooth storage/retrieval
