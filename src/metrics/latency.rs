//! Tokenizer latency metrics.

use std::time::Duration;

/// Latency accumulators for tokenizer operations.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TokenizerLatency {
    /// Batch tokenization latency.
    pub batch: Duration,
    /// Template formatting latency.
    pub template_format: Duration,
    /// Tokenizer encode latency.
    pub tokenizer_encode: Duration,
    /// Cache lookup latency.
    pub cache_lookup: Duration,
    /// Cache store latency.
    pub cache_store: Duration,
    /// Queue wait time.
    pub queue_wait: Duration,
}

impl TokenizerLatency {
    /// Records batch tokenization latency.
    pub fn record_batch(&mut self, latency: Duration) {
        self.batch = self.batch.saturating_add(latency);
    }

    /// Records template formatting latency.
    pub fn record_template_format(&mut self, latency: Duration) {
        self.template_format = self.template_format.saturating_add(latency);
    }

    /// Records tokenizer encoding latency.
    pub fn record_tokenizer_encode(&mut self, latency: Duration) {
        self.tokenizer_encode = self.tokenizer_encode.saturating_add(latency);
    }

    /// Records cache lookup latency.
    pub fn record_cache_lookup(&mut self, latency: Duration) {
        self.cache_lookup = self.cache_lookup.saturating_add(latency);
    }

    /// Records cache store latency.
    pub fn record_cache_store(&mut self, latency: Duration) {
        self.cache_store = self.cache_store.saturating_add(latency);
    }

    /// Records queue wait latency.
    pub fn record_queue_wait(&mut self, latency: Duration) {
        self.queue_wait = self.queue_wait.saturating_add(latency);
    }
}
