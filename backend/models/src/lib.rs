pub mod auth;
pub mod comment;
pub mod company;
pub mod error;
pub mod profile;
pub mod user;

use std::collections::HashMap;
pub type DeleteWrapper = HashMap<(), ()>;
