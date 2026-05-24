//! Distributed tokenizer worker contract.

use crate::{
    stream::{RawSampleBatch, TokenizerStreamAdapter},
    tokenizer::TunexTokenizer,
    TokenizerResult,
};

use super::{DistributedTokenBatch, TokenizerWorkerAssignment};

/// Stable tokenizer worker id.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TokenizerWorkerId(pub u64);

/// Rank-aware tokenizer worker wrapper.
pub struct DistributedTokenizerWorker<T> {
    assignment: TokenizerWorkerAssignment,
    adapter: TokenizerStreamAdapter<T>,
}

impl<T> DistributedTokenizerWorker<T>
where
    T: TunexTokenizer,
{
    /// Creates a distributed tokenizer worker.
    #[must_use]
    pub const fn new(
        assignment: TokenizerWorkerAssignment,
        adapter: TokenizerStreamAdapter<T>,
    ) -> Self {
        Self {
            assignment,
            adapter,
        }
    }

    /// Returns worker assignment.
    #[must_use]
    pub const fn assignment(&self) -> &TokenizerWorkerAssignment {
        &self.assignment
    }

    /// Tokenizes one owned batch using shard-ownership routing.
    ///
    /// # Errors
    ///
    /// Returns an error when routing or tokenization fails.
    pub fn tokenize_owned_batch(
        &mut self,
        batch: RawSampleBatch,
    ) -> TokenizerResult<DistributedTokenBatch> {
        let route = self
            .assignment
            .router(super::RoutingStrategy::ShardOwnership)
            .route(&batch)?;
        let envelope = self.adapter.tokenize_batch(batch)?;
        let distributed = DistributedTokenBatch::new(route, self.assignment.epoch, envelope);
        distributed.validate_for_rank(self.assignment.rank)?;
        Ok(distributed)
    }
}
