#[derive(thiserror::Error, Debug)]
pub enum CacheError {
    #[error("An unknown error has occured")]
    Unknown,
    #[error("Redis error: {0}")]
    Redis(#[from] deadpool_redis::redis::RedisError),
    #[error("Redis pool error: {0}")]
    Pool(#[from] deadpool_redis::PoolError),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Failed to create Redis pool: {0}")]
    CreatePool(#[from] deadpool_redis::CreatePoolError),
}
