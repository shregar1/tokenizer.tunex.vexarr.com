//! Parallel tokenization worker contract.

use std::time::Instant;

use tunex_core::types::{RawSample, TokenizedSample};

use crate::{TokenizedBatch, TokenizerResult};

use super::{SampleChunk, TunexTokenizer, WorkerTokenizationStats};

/// Stateless worker descriptor used for metrics attribution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TokenizationWorker {
    /// Worker id.
    pub worker_id: usize,
}

impl TokenizationWorker {
    /// Creates a worker descriptor.
    #[must_use]
    pub const fn new(worker_id: usize) -> Self {
        Self { worker_id }
    }

    /// Tokenizes one chunk and returns samples with worker stats.
    pub fn tokenize_chunk<T>(
        self,
        tokenizer: &T,
        samples: &[RawSample],
        chunk: SampleChunk,
    ) -> TokenizerResult<(usize, Vec<TokenizedSample>, WorkerTokenizationStats)>
    where
        T: TunexTokenizer,
    {
        let started = Instant::now();
        let TokenizedBatch { samples, metadata } = tokenizer.tokenize_batch(samples)?;
        let stats = WorkerTokenizationStats {
            worker_id: self.worker_id,
            chunks: 1,
            samples: metadata.samples as u64,
            tokens: metadata.tokens as u64,
            elapsed: started.elapsed(),
        };

        Ok((chunk.index, samples, stats))
    }
}
