//! Parallel batch tokenization orchestration.

use std::time::Instant;

use rayon::prelude::*;
use tunex_core::types::RawSample;

use crate::{TokenizedBatch, TokenizerError, TokenizerResult};

use super::{
    BatchTokenizationScheduler, ParallelTokenizationStats, TokenizationWorker, TunexTokenizer,
    WorkerTokenizationStats,
};

/// Parallel batch tokenizer wrapper.
pub struct ParallelBatchTokenizer<'a, T> {
    tokenizer: &'a T,
    scheduler: BatchTokenizationScheduler,
}

impl<'a, T> ParallelBatchTokenizer<'a, T>
where
    T: TunexTokenizer,
{
    /// Creates a parallel tokenizer wrapper.
    #[must_use]
    pub const fn new(tokenizer: &'a T, scheduler: BatchTokenizationScheduler) -> Self {
        Self {
            tokenizer,
            scheduler,
        }
    }

    /// Tokenizes samples in chunks while preserving input order.
    pub fn tokenize_ordered(
        &self,
        samples: &[RawSample],
    ) -> TokenizerResult<(TokenizedBatch, ParallelTokenizationStats)> {
        let started = Instant::now();
        let plan = self.scheduler.plan(samples.len());
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(plan.workers)
            .build()
            .map_err(|error| TokenizerError::Parallel(error.to_string()))?;

        let mut chunks = pool.install(|| {
            plan.chunks
                .par_iter()
                .enumerate()
                .map(|(worker_id, chunk)| {
                    let worker = TokenizationWorker::new(worker_id % plan.workers);
                    worker.tokenize_chunk(self.tokenizer, &samples[chunk.start..chunk.end], *chunk)
                })
                .collect::<TokenizerResult<Vec<_>>>()
        })?;

        chunks.sort_by_key(|(index, _, _)| *index);
        let mut outputs = Vec::with_capacity(samples.len());
        let mut worker_stats = Vec::<WorkerTokenizationStats>::with_capacity(chunks.len());
        for (_, mut chunk_samples, stats) in chunks {
            outputs.append(&mut chunk_samples);
            worker_stats.push(stats);
        }

        let batch = TokenizedBatch::new(0, outputs);
        let stats = ParallelTokenizationStats {
            workers: plan.workers,
            chunks: plan.chunks.len() as u64,
            samples: batch.metadata.samples as u64,
            tokens: batch.metadata.tokens as u64,
            latency: started.elapsed(),
            worker_stats,
        };

        Ok((batch, stats))
    }
}
