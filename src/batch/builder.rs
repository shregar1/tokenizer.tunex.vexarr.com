//! Builder for the flat tokenized batch layout.

use tunex_core::types::TokenizedSample;

use super::{TokenizedBatch, TokenizedBatchMetadata};

/// Reusable builder for [`TokenizedBatch`].
#[derive(Clone, Debug)]
pub struct TokenizedBatchBuilder {
    batch_id: u64,
    input_ids: Vec<u32>,
    offsets: Vec<u32>,
    lengths: Vec<u32>,
    attention_mask: Vec<u8>,
    sample_ids: Vec<Option<String>>,
}

impl TokenizedBatchBuilder {
    /// Creates a builder with sample capacity.
    #[must_use]
    pub fn with_capacity(batch_id: u64, samples: usize) -> Self {
        Self {
            batch_id,
            input_ids: Vec::new(),
            offsets: Vec::with_capacity(samples),
            lengths: Vec::with_capacity(samples),
            attention_mask: Vec::new(),
            sample_ids: Vec::with_capacity(samples),
        }
    }

    /// Pushes one tokenized sample.
    pub fn push(&mut self, sample: TokenizedSample) {
        self.offsets.push(
            u32::try_from(self.input_ids.len()).expect("tokenized batch offset exceeds u32::MAX"),
        );
        self.lengths.push(
            u32::try_from(sample.token_ids.len())
                .expect("tokenized sample length exceeds u32::MAX"),
        );
        self.sample_ids.push(sample.id);
        self.attention_mask.extend(sample.attention_mask);
        self.input_ids.extend(sample.token_ids);
    }

    /// Finishes the batch.
    #[must_use]
    pub fn finish(self) -> TokenizedBatch {
        TokenizedBatch {
            metadata: TokenizedBatchMetadata {
                batch_id: self.batch_id,
                created_at: std::time::SystemTime::now(),
                samples: self.lengths.len(),
                tokens: self.input_ids.len(),
            },
            input_ids: self.input_ids,
            offsets: self.offsets,
            lengths: self.lengths,
            attention_mask: self.attention_mask,
            sample_ids: self.sample_ids,
        }
    }
}
