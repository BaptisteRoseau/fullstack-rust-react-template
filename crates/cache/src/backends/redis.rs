use config::Config;

use crate::{Cache, error::CacheError};

// TODO: deadpool_redis :D
//
pub struct Redis {
    timeout_s: Option<u32>,
}

impl Redis {
    pub fn new(timeout_s: Option<u32>) -> Self {
        Self { timeout_s }
    }
}

impl TryFrom<&Config> for Redis {
    type Error = CacheError;

    fn try_from(_value: &Config) -> Result<Self, Self::Error> {
        // TODO: Actually implement the cache
        Ok(Self { timeout_s: None })
    }
}

// impl Cache for Redis {}
