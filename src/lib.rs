mod storage;

#[cfg(feature = "memory_backing")]
pub use storage::memory::MemoryTagIndex;

#[cfg(feature = "redis_backing")]
pub use storage::redis::RedisTagIndex;