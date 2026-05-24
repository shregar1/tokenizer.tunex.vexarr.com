//! Tokenizer counters.

/// Monotonic counters collected by tokenizer components.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TokenizerCounters {
    /// Samples tokenized.
    pub samples: u64,
    /// Tokens emitted.
    pub tokens: u64,
    /// Cache hits.
    pub cache_hits: u64,
    /// Cache misses.
    pub cache_misses: u64,
    /// Worker busy ticks.
    pub worker_busy_ticks: u64,
    /// Worker total ticks.
    pub worker_total_ticks: u64,
    /// Allocation count.
    pub allocations: u64,
    /// Truncated samples.
    pub truncations: u64,
    /// Tokenization errors.
    pub errors: u64,
}

impl TokenizerCounters {
    /// Records processed samples and tokens.
    pub fn record_batch(&mut self, samples: u64, tokens: u64) {
        self.samples = self.samples.saturating_add(samples);
        self.tokens = self.tokens.saturating_add(tokens);
    }

    /// Records cache lookup result.
    pub fn record_cache_lookup(&mut self, hit: bool) {
        if hit {
            self.cache_hits = self.cache_hits.saturating_add(1);
        } else {
            self.cache_misses = self.cache_misses.saturating_add(1);
        }
    }

    /// Records worker utilization ticks.
    pub fn record_worker_ticks(&mut self, busy: u64, total: u64) {
        self.worker_busy_ticks = self.worker_busy_ticks.saturating_add(busy);
        self.worker_total_ticks = self.worker_total_ticks.saturating_add(total);
    }

    /// Records one error.
    pub fn record_error(&mut self) {
        self.errors = self.errors.saturating_add(1);
    }

    /// Returns cache hit ratio.
    #[must_use]
    pub fn cache_hit_ratio(&self) -> f64 {
        let total = self.cache_hits.saturating_add(self.cache_misses);
        if total == 0 {
            return 0.0;
        }
        self.cache_hits as f64 / total as f64
    }

    /// Returns worker utilization.
    #[must_use]
    pub fn worker_utilization(&self) -> f64 {
        if self.worker_total_ticks == 0 {
            return 0.0;
        }
        self.worker_busy_ticks as f64 / self.worker_total_ticks as f64
    }

    /// Returns average tokens per sample.
    #[must_use]
    pub fn average_tokens_per_sample(&self) -> f64 {
        if self.samples == 0 {
            return 0.0;
        }
        self.tokens as f64 / self.samples as f64
    }
}
