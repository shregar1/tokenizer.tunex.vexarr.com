//! Distributed tokenizer worker assignment.

use crate::{stream::TokenizedBatchEnvelope, TokenizerResult};

use super::{
    BatchRoute, BatchRouter, DistributedTokenizerEpoch, DistributedTokenizerRank, RoutingStrategy,
    TokenizerShard, TokenizerWorkerId,
};

/// Worker assignment for one tokenizer rank.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenizerWorkerAssignment {
    /// Assigned worker id.
    pub worker_id: TokenizerWorkerId,
    /// Assigned rank.
    pub rank: DistributedTokenizerRank,
    /// Synchronized epoch.
    pub epoch: DistributedTokenizerEpoch,
    /// Owned shards.
    pub shards: Vec<TokenizerShard>,
}

impl TokenizerWorkerAssignment {
    /// Creates a worker assignment.
    #[must_use]
    pub fn new(
        worker_id: TokenizerWorkerId,
        rank: DistributedTokenizerRank,
        epoch: DistributedTokenizerEpoch,
        shards: Vec<TokenizerShard>,
    ) -> Self {
        Self {
            worker_id,
            rank,
            epoch,
            shards,
        }
    }

    /// Creates a router for this assignment.
    #[must_use]
    pub fn router(&self, strategy: RoutingStrategy) -> BatchRouter {
        BatchRouter::new(strategy, self.rank, self.shards.clone())
    }
}

/// Distributed-safe tokenized batch envelope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DistributedTokenBatch {
    /// Route used to create this batch.
    pub route: BatchRoute,
    /// Deterministic distributed output id.
    pub distributed_batch_id: u64,
    /// Tokenized stream envelope.
    pub envelope: TokenizedBatchEnvelope,
}

impl DistributedTokenBatch {
    /// Creates a distributed token batch.
    #[must_use]
    pub fn new(
        route: BatchRoute,
        epoch: DistributedTokenizerEpoch,
        envelope: TokenizedBatchEnvelope,
    ) -> Self {
        let distributed_batch_id = distributed_batch_id(
            epoch.epoch,
            route.rank.rank as u64,
            envelope.input_batch_id,
            envelope.output_batch_id,
        );
        Self {
            route,
            distributed_batch_id,
            envelope,
        }
    }

    /// Validates route ownership.
    pub fn validate_for_rank(&self, rank: DistributedTokenizerRank) -> TokenizerResult<()> {
        if self.route.rank != rank {
            return Err(crate::TokenizerError::InvalidConfig(format!(
                "distributed batch routed to rank {}, not rank {}",
                self.route.rank.rank, rank.rank
            )));
        }
        Ok(())
    }
}

/// Derives a stable distributed batch id.
#[must_use]
pub const fn distributed_batch_id(
    epoch: u64,
    rank: u64,
    input_batch_id: u64,
    output_batch_id: u64,
) -> u64 {
    (epoch << 48) ^ (rank << 40) ^ (input_batch_id << 8) ^ output_batch_id
}
