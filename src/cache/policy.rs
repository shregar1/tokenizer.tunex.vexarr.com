//! Tokenizer cache policies.

use std::path::PathBuf;

/// Cache eviction algorithm.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CacheEvictionPolicy {
    /// Least recently used eviction.
    Lru,
    /// Least frequently used eviction placeholder.
    Lfu,
}

/// Cache storage backend.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CacheStorage {
    /// Keep entries only in memory.
    MemoryOnly,
    /// Allow disk persistence.
    DiskBacked(PathBuf),
}

/// Tokenizer cache policy.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CachePolicy {
    /// Disable caching.
    Disabled,
    /// Bounded in-memory cache.
    MemoryOnly {
        /// Maximum number of cache entries.
        max_entries: usize,
        /// Eviction policy.
        eviction: CacheEvictionPolicy,
    },
    /// Disk-backed cache contract.
    DiskBacked {
        /// Cache directory.
        directory: PathBuf,
        /// Maximum in-memory entries.
        max_memory_entries: usize,
        /// Eviction policy.
        eviction: CacheEvictionPolicy,
    },
}

impl CachePolicy {
    /// Returns true when caching is enabled.
    #[must_use]
    pub const fn is_enabled(&self) -> bool {
        !matches!(self, Self::Disabled)
    }
}

impl Default for CachePolicy {
    fn default() -> Self {
        Self::MemoryOnly {
            max_entries: 16_384,
            eviction: CacheEvictionPolicy::Lru,
        }
    }
}
