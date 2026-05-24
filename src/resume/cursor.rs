//! Tokenizer cursor tracking.

use crate::distributed::{DistributedTokenizerEpoch, DistributedTokenizerRank};

/// Exact tokenizer position for checkpoint resume.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenizerCursor {
    /// Current epoch.
    pub epoch: u64,
    /// Logical sample index in tokenizer order.
    pub sample_index: u64,
    /// Logical batch index in tokenizer order.
    pub batch_index: u64,
    /// Distributed shard id.
    pub shard_id: Option<u32>,
    /// Distributed rank.
    pub rank: Option<u32>,
    /// Tokenizer model/config fingerprint.
    pub tokenizer_fingerprint: String,
    /// Chat template fingerprint.
    pub template_fingerprint: String,
    /// Deterministic seed.
    pub seed: u64,
}

impl TokenizerCursor {
    /// Creates a tokenizer cursor at epoch start.
    #[must_use]
    pub fn new(
        epoch: u64,
        seed: u64,
        tokenizer_fingerprint: impl Into<String>,
        template_fingerprint: impl Into<String>,
    ) -> Self {
        Self {
            epoch,
            sample_index: 0,
            batch_index: 0,
            shard_id: None,
            rank: None,
            tokenizer_fingerprint: tokenizer_fingerprint.into(),
            template_fingerprint: template_fingerprint.into(),
            seed,
        }
    }

    /// Adds distributed rank identity.
    #[must_use]
    pub const fn with_rank(mut self, rank: DistributedTokenizerRank, shard_id: u32) -> Self {
        self.rank = Some(rank.rank as u32);
        self.shard_id = Some(shard_id);
        self
    }

    /// Advances tokenizer position by one consumed batch.
    #[must_use]
    pub const fn advance_batch(mut self, samples: u64) -> Self {
        self.sample_index += samples;
        self.batch_index += 1;
        self
    }

    /// Converts this cursor into a distributed tokenizer epoch descriptor.
    #[must_use]
    pub const fn distributed_epoch(&self) -> DistributedTokenizerEpoch {
        DistributedTokenizerEpoch::new(self.epoch, self.seed)
    }
}

impl Default for TokenizerCursor {
    fn default() -> Self {
        Self::new(0, 42, "unknown-tokenizer", "none")
    }
}
