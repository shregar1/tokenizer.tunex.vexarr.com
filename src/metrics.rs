//! Tokenizer metrics contracts.

use std::time::Duration;

/// Tokenizer throughput and latency counters.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TokenizerMetrics {
    /// Samples tokenized.
    pub samples: u64,
    /// Tokens emitted.
    pub tokens: u64,
    /// Total tokenization latency.
    pub tokenize_latency: Duration,
    /// Batch tokenization latency.
    pub batch_latency: Duration,
}

impl TokenizerMetrics {
    /// Returns tokens per second for total latency.
    #[must_use]
    pub fn tokens_per_second(&self) -> f64 {
        let seconds = self.tokenize_latency.as_secs_f64();
        if seconds == 0.0 {
            return 0.0;
        }

        self.tokens as f64 / seconds
    }
}
