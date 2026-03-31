use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::Permissions;

//TODO: Use this to build object permissions

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Role {
    name: String,
    id: Uuid,
    grants: HashSet<Permissions>,
    forbids: HashSet<Permissions>,
}
