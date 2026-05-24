//! Disk-backed tokenizer cache contract.

use std::path::{Path, PathBuf};

use super::{CachePolicy, MemoryTokenizerCache, TokenizerCacheEntry, TokenizerCacheKey};

/// Disk-backed cache facade.
///
/// The v0.1 layer defines the public contract and keeps a bounded memory front
/// cache. Durable serialization is intentionally left for the serialization
/// phase so the on-disk format does not freeze too early.
#[derive(Clone, Debug)]
pub struct DiskTokenizerCache {
    directory: PathBuf,
    memory: MemoryTokenizerCache,
}

impl DiskTokenizerCache {
    /// Creates a disk-backed cache facade.
    #[must_use]
    pub fn new(directory: impl Into<PathBuf>, max_memory_entries: usize) -> Self {
        Self {
            directory: directory.into(),
            memory: MemoryTokenizerCache::new(max_memory_entries),
        }
    }

    /// Creates a cache from a disk-backed policy.
    #[must_use]
    pub fn from_policy(policy: &CachePolicy) -> Option<Self> {
        match policy {
            CachePolicy::DiskBacked {
                directory,
                max_memory_entries,
                ..
            } => Some(Self::new(directory.clone(), *max_memory_entries)),
            CachePolicy::Disabled | CachePolicy::MemoryOnly { .. } => None,
        }
    }

    /// Returns cache directory.
    #[must_use]
    pub fn directory(&self) -> &Path {
        &self.directory
    }

    /// Looks up a cache entry in the memory front cache.
    pub fn get(&mut self, key: &TokenizerCacheKey) -> Option<TokenizerCacheEntry> {
        self.memory.get(key)
    }

    /// Inserts an entry into the memory front cache.
    pub fn insert(&mut self, key: TokenizerCacheKey, entry: TokenizerCacheEntry) {
        self.memory.insert(key, entry);
    }

    /// Returns the memory front cache.
    #[must_use]
    pub const fn memory(&self) -> &MemoryTokenizerCache {
        &self.memory
    }
}
