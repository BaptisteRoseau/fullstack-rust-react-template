use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use super::profile::ProfileInfo;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CommentInfo {
    pub id: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub body: String,
    pub author: ProfileInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CommentInfoWrapper {
    pub comment: CommentInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CommentCreateInfo {
    pub body: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommentCreateInfoWrapper {
    pub comment: CommentCreateInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommentListInfo {
    pub comments: Vec<CommentInfo>,
}
