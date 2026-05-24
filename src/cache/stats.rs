//! Tokenizer cache metrics.

/// Tokenizer cache counters.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TokenizerCacheStats {
    /// Cache lookups.
    pub lookups: u64,
    /// Cache hits.
    pub hits: u64,
    /// Cache misses.
    pub misses: u64,
    /// Inserted entries.
    pub inserts: u64,
    /// Evicted entries.
    pub evictions: u64,
    /// Estimated bytes retained by cache entries.
    pub retained_bytes: u64,
}

impl TokenizerCacheStats {
    /// Records a cache hit.
    pub fn record_hit(&mut self) {
        self.lookups = self.lookups.saturating_add(1);
        self.hits = self.hits.saturating_add(1);
    }

    /// Records a cache miss.
    pub fn record_miss(&mut self) {
        self.lookups = self.lookups.saturating_add(1);
        self.misses = self.misses.saturating_add(1);
    }

    /// Records an insert.
    pub fn record_insert(&mut self, bytes: u64) {
        self.inserts = self.inserts.saturating_add(1);
        self.retained_bytes = self.retained_bytes.saturating_add(bytes);
    }

    /// Records an eviction.
    pub fn record_eviction(&mut self, bytes: u64) {
        self.evictions = self.evictions.saturating_add(1);
        self.retained_bytes = self.retained_bytes.saturating_sub(bytes);
    }

    /// Returns cache hit ratio.
    #[must_use]
    pub fn hit_ratio(&self) -> f64 {
        if self.lookups == 0 {
            return 0.0;
        }
        self.hits as f64 / self.lookups as f64
    }
}
