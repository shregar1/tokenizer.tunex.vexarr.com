//! Batch tokenization output.

use std::time::SystemTime;

use tunex_core::types::TokenizedSample;

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

/// Batch of tokenized samples.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenizedBatch {
    /// Tokenized samples.
    pub samples: Vec<TokenizedSample>,
    /// Batch metadata.
    pub metadata: TokenizedBatchMetadata,
}

impl TokenizedBatch {
    /// Creates a tokenized batch.
    #[must_use]
    pub fn new(batch_id: u64, samples: Vec<TokenizedSample>) -> Self {
        let tokens = samples.iter().map(|sample| sample.token_ids.len()).sum();
        Self {
            metadata: TokenizedBatchMetadata {
                batch_id,
                created_at: SystemTime::now(),
                samples: samples.len(),
                tokens,
            },
            samples,
        }
    }

    /// Returns sample count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.samples.len()
    }

    /// Returns true when the batch is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }
}
