//! Tokenizer throughput metrics.

use std::time::Duration;

/// Throughput calculations for tokenizer work.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TokenizerThroughput {
    /// Samples per second.
    pub samples_per_sec: f64,
    /// Tokens per second.
    pub tokens_per_sec: f64,
}

impl TokenizerThroughput {
    /// Computes throughput from samples, tokens, and elapsed latency.
    #[must_use]
    pub fn from_counts(samples: u64, tokens: u64, elapsed: Duration) -> Self {
        let seconds = elapsed.as_secs_f64();
        if seconds == 0.0 {
            return Self::default();
        }

        Self {
            samples_per_sec: samples as f64 / seconds,
            tokens_per_sec: tokens as f64 / seconds,
        }
    }
}
