//! Deterministic sample chunk scheduling.

use crate::{TokenizerError, TokenizerResult};

/// One contiguous sample chunk.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SampleChunk {
    /// Chunk ordinal.
    pub index: usize,
    /// Start sample index, inclusive.
    pub start: usize,
    /// End sample index, exclusive.
    pub end: usize,
}

impl SampleChunk {
    /// Returns chunk length.
    #[must_use]
    pub const fn len(self) -> usize {
        self.end - self.start
    }

    /// Returns true when chunk is empty.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }
}

/// Full deterministic tokenization plan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BatchTokenizationPlan {
    /// Planned chunks.
    pub chunks: Vec<SampleChunk>,
    /// Requested worker count.
    pub workers: usize,
}

/// Bounded chunk scheduler for tokenizer batches.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BatchTokenizationScheduler {
    /// Samples per chunk.
    pub chunk_size: usize,
    /// Maximum Rayon workers to use.
    pub max_workers: usize,
}

impl BatchTokenizationScheduler {
    /// Creates a scheduler.
    pub fn new(chunk_size: usize, max_workers: usize) -> TokenizerResult<Self> {
        if chunk_size == 0 {
            return Err(TokenizerError::InvalidConfig(
                "tokenizer chunk_size must be greater than zero".to_owned(),
            ));
        }
        if max_workers == 0 {
            return Err(TokenizerError::InvalidConfig(
                "tokenizer workers must be greater than zero".to_owned(),
            ));
        }

        Ok(Self {
            chunk_size,
            max_workers,
        })
    }

    /// Builds a deterministic chunk plan.
    #[must_use]
    pub fn plan(&self, samples: usize) -> BatchTokenizationPlan {
        let mut chunks = Vec::with_capacity(samples.div_ceil(self.chunk_size));
        for (index, start) in (0..samples).step_by(self.chunk_size).enumerate() {
            chunks.push(SampleChunk {
                index,
                start,
                end: start.saturating_add(self.chunk_size).min(samples),
            });
        }

        BatchTokenizationPlan {
            chunks,
            workers: self.max_workers,
        }
    }
}
