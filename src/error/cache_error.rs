use thiserror::Error;
#[derive(Debug, Error)]
pub enum CacheError {
    #[error("Item not found in cache")]
    CacheMiss,
    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),
}