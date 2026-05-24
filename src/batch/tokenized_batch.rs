//! Flat tokenized batch representation.

use std::time::SystemTime;

use tunex_core::types::TokenizedSample;

use super::TokenizedBatchView;

/// Contiguous token id buffer.
pub type TokenBuffer = Vec<u32>;

/// Per-sample token offsets into `input_ids`.
pub type TokenOffsets = Vec<u32>;

/// Tokenized batch metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenizedBatchMetadata {
    /// Batch id.
    pub batch_id: u64,
    /// Creation timestamp.
    pub created_at: SystemTime,
    /// Number of input samples.
    pub samples: usize,
    /// Total emitted tokens.
    pub tokens: usize,
}

/// Cache-friendly tokenized batch.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenizedBatch {
    /// Flattened input ids for all samples.
    pub input_ids: TokenBuffer,
    /// Start offset per sample.
    pub offsets: TokenOffsets,
    /// Length per sample.
    pub lengths: Vec<u32>,
    /// Flattened attention mask aligned with `input_ids`.
    pub attention_mask: Vec<u8>,
    /// Optional sample ids aligned with offsets.
    pub sample_ids: Vec<Option<String>>,
    /// Batch metadata.
    pub metadata: TokenizedBatchMetadata,
}

impl TokenizedBatch {
    /// Creates a flat tokenized batch from row-oriented samples.
    #[must_use]
    pub fn new(batch_id: u64, samples: Vec<TokenizedSample>) -> Self {
        let mut builder = super::TokenizedBatchBuilder::with_capacity(batch_id, samples.len());
        for sample in samples {
            builder.push(sample);
        }
        builder.finish()
    }

    /// Returns sample count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.lengths.len()
    }

    /// Returns true when the batch is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.lengths.is_empty()
    }

    /// Returns a borrowed batch view.
    #[must_use]
    pub const fn view(&self) -> TokenizedBatchView<'_> {
        TokenizedBatchView { batch: self }
    }

    /// Converts the flat layout back to core row-oriented samples.
    #[must_use]
    pub fn to_samples(&self) -> Vec<TokenizedSample> {
        (0..self.len())
            .map(|index| {
                let view = self.view().sample(index).expect("sample index from len");
                TokenizedSample {
                    id: self.sample_ids[index].clone(),
                    token_ids: view.input_ids.to_vec(),
                    attention_mask: view.attention_mask.to_vec(),
                }
            })
            .collect()
    }
}
