//! Tokenizer cache contracts and in-memory cache implementation.

pub mod disk;
pub mod key;
pub mod memory;
pub mod policy;
pub mod stats;

pub use disk::DiskTokenizerCache;
pub use key::{CacheTarget, TokenizerCacheKey, TokenizerFingerprint};
pub use memory::{MemoryTokenizerCache, TokenizerCacheEntry};
pub use policy::{CacheEvictionPolicy, CachePolicy, CacheStorage};
pub use stats::TokenizerCacheStats;
