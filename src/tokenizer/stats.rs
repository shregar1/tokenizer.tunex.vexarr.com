//! Parallel tokenization statistics.

use std::time::Duration;

/// Worker-level tokenizer stats.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WorkerTokenizationStats {
    /// Worker id.
    pub worker_id: usize,
    /// Chunks processed.
    pub chunks: u64,
    /// Samples processed.
    pub samples: u64,
    /// Tokens emitted.
    pub tokens: u64,
    /// Worker elapsed time.
    pub elapsed: Duration,
}

/// Aggregate parallel tokenization stats.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ParallelTokenizationStats {
    /// Worker count.
    pub workers: usize,
    /// Chunk count.
    pub chunks: u64,
    /// Samples processed.
    pub samples: u64,
    /// Tokens emitted.
    pub tokens: u64,
    /// Batch latency.
    pub latency: Duration,
    /// Per-worker stats.
    pub worker_stats: Vec<WorkerTokenizationStats>,
}

impl ParallelTokenizationStats {
    /// Returns samples per second.
    #[must_use]
    pub fn samples_per_second(&self) -> f64 {
        let seconds = self.latency.as_secs_f64();
        if seconds == 0.0 {
            return 0.0;
        }

        self.samples as f64 / seconds
    }

    /// Returns tokens per second.
    #[must_use]
    pub fn tokens_per_second(&self) -> f64 {
        let seconds = self.latency.as_secs_f64();
        if seconds == 0.0 {
            return 0.0;
        }

        self.tokens as f64 / seconds
    }
}
