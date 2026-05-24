//! In-memory tokenizer cache.

use std::collections::{BTreeMap, VecDeque};

use crate::{cache::TokenizerCacheStats, TokenizedBatch};

use super::TokenizerCacheKey;

/// Cached tokenizer artifact.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TokenizerCacheEntry {
    /// Token ids for one text input.
    TokenIds(Vec<u32>),
    /// Template-rendered text.
    TemplateOutput(String),
    /// Complete tokenized batch.
    TokenizedBatch(TokenizedBatch),
}

impl TokenizerCacheEntry {
    /// Estimated retained bytes for stats and eviction accounting.
    #[must_use]
    pub fn estimated_bytes(&self) -> u64 {
        match self {
            Self::TokenIds(ids) => (ids.len() * std::mem::size_of::<u32>()) as u64,
            Self::TemplateOutput(text) => text.len() as u64,
            Self::TokenizedBatch(batch) => {
                ((batch.input_ids.len() * std::mem::size_of::<u32>())
                    + batch.offsets.len() * std::mem::size_of::<u32>()
                    + batch.lengths.len() * std::mem::size_of::<u32>()
                    + batch.attention_mask.len()) as u64
            }
        }
    }
}

/// Bounded in-memory LRU tokenizer cache.
#[derive(Clone, Debug)]
pub struct MemoryTokenizerCache {
    max_entries: usize,
    entries: BTreeMap<TokenizerCacheKey, TokenizerCacheEntry>,
    recency: VecDeque<TokenizerCacheKey>,
    stats: TokenizerCacheStats,
}

impl MemoryTokenizerCache {
    /// Creates a bounded memory cache.
    #[must_use]
    pub fn new(max_entries: usize) -> Self {
        Self {
            max_entries,
            entries: BTreeMap::new(),
            recency: VecDeque::new(),
            stats: TokenizerCacheStats::default(),
        }
    }

    /// Returns cache entry count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true when the cache has no entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns immutable cache stats.
    #[must_use]
    pub const fn stats(&self) -> TokenizerCacheStats {
        self.stats
    }

    /// Looks up and clones a cache entry.
    pub fn get(&mut self, key: &TokenizerCacheKey) -> Option<TokenizerCacheEntry> {
        let entry = self.entries.get(key).cloned();
        if entry.is_some() {
            self.stats.record_hit();
            self.touch(key);
        } else {
            self.stats.record_miss();
        }
        entry
    }

    /// Inserts or replaces a cache entry.
    pub fn insert(&mut self, key: TokenizerCacheKey, entry: TokenizerCacheEntry) {
        if self.max_entries == 0 {
            return;
        }

        if let Some(old) = self.entries.insert(key.clone(), entry.clone()) {
            self.stats.record_eviction(old.estimated_bytes());
        }
        self.stats.record_insert(entry.estimated_bytes());
        self.touch(&key);
        self.evict_until_bounded();
    }

    fn touch(&mut self, key: &TokenizerCacheKey) {
        self.recency.retain(|candidate| candidate != key);
        self.recency.push_back(key.clone());
    }

    fn evict_until_bounded(&mut self) {
        while self.entries.len() > self.max_entries {
            if let Some(key) = self.recency.pop_front() {
                if let Some(entry) = self.entries.remove(&key) {
                    self.stats.record_eviction(entry.estimated_bytes());
                }
            } else {
                break;
            }
        }
    }
}
