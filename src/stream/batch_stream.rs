//! Batch stream types for dataset-to-tokenizer flow.

use tunex_core::types::RawSample;

use crate::{TokenizedBatch, TokenizerStreamStats};

/// Raw sample batch consumed from a dataset module.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RawSampleBatch {
    /// Input batch id from the dataset pipeline.
    pub batch_id: u64,
    /// Raw samples in input order.
    pub samples: Vec<RawSample>,
    /// True when this is a final or otherwise partial batch.
    pub partial: bool,
}

impl RawSampleBatch {
    /// Creates a raw sample batch.
    #[must_use]
    pub const fn new(batch_id: u64, samples: Vec<RawSample>, partial: bool) -> Self {
        Self {
            batch_id,
            samples,
            partial,
        }
    }

    /// Returns sample count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.samples.len()
    }

    /// Returns true when there are no samples.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }
}

/// Tokenized batch plus stream correlation metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenizedBatchEnvelope {
    /// Input raw sample batch id.
    pub input_batch_id: u64,
    /// Output tokenized batch id.
    pub output_batch_id: u64,
    /// Tokenized batch in input order.
    pub batch: TokenizedBatch,
    /// Stream stats snapshot after this batch.
    pub stats: TokenizerStreamStats,
}
