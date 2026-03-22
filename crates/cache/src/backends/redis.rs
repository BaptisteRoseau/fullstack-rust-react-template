use crate::Cache;

// TODO: deadpool_redis :D

pub struct Redis {
    timeout_s: Option<u32>,
}

impl Redis {
    pub fn new(timeout_s: Option<u32>) -> Self {
        Self { timeout_s }
    }
}

// impl Cache for Redis {}
