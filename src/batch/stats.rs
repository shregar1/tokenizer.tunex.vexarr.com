//! Tokenized batch layout statistics.

use super::TokenizedBatch;

/// Tokenized batch memory statistics.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TokenizedBatchStats {
    /// Sample count.
    pub samples: usize,
    /// Token count.
    pub tokens: usize,
    /// Approximate backing bytes.
    pub backing_bytes: usize,
}

impl TokenizedBatchStats {
    /// Creates stats from a batch.
    #[must_use]
    pub fn from_batch(batch: &TokenizedBatch) -> Self {
        Self {
            samples: batch.len(),
            tokens: batch.input_ids.len(),
            backing_bytes: batch.input_ids.len() * std::mem::size_of::<u32>()
                + batch.offsets.len() * std::mem::size_of::<u32>()
                + batch.lengths.len() * std::mem::size_of::<u32>()
                + batch.attention_mask.len() * std::mem::size_of::<u8>(),
        }
    }
}
