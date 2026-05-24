//! Distributed batch routing.

use crate::{stream::RawSampleBatch, TokenizerError, TokenizerResult};

use super::{DistributedTokenizerRank, TokenizerShard};

/// Routing mode for tokenization batches.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RoutingStrategy {
    /// Route by dataset shard ownership.
    ShardOwnership,
    /// Route by input batch id modulo world size.
    RoundRobin,
    /// Route all batches to the current rank.
    LocalOnly,
}

/// Routing decision for one raw batch.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BatchRoute {
    /// Input batch id.
    pub input_batch_id: u64,
    /// Destination rank.
    pub rank: DistributedTokenizerRank,
    /// Dataset shard id when known.
    pub shard_id: Option<u64>,
}

/// Deterministic tokenizer batch router.
#[derive(Clone, Debug)]
pub struct BatchRouter {
    strategy: RoutingStrategy,
    rank: DistributedTokenizerRank,
    shards: Vec<TokenizerShard>,
}

impl BatchRouter {
    /// Creates a router.
    #[must_use]
    pub fn new(
        strategy: RoutingStrategy,
        rank: DistributedTokenizerRank,
        shards: Vec<TokenizerShard>,
    ) -> Self {
        Self {
            strategy,
            rank,
            shards,
        }
    }

    /// Routes one input batch deterministically.
    pub fn route(&self, batch: &RawSampleBatch) -> TokenizerResult<BatchRoute> {
        match self.strategy {
            RoutingStrategy::LocalOnly => Ok(BatchRoute {
                input_batch_id: batch.batch_id,
                rank: self.rank,
                shard_id: self
                    .shards
                    .iter()
                    .find(|shard| shard.rank == self.rank)
                    .map(|shard| shard.shard_id),
            }),
            RoutingStrategy::RoundRobin => {
                let target = usize::try_from(batch.batch_id % self.rank.world_size as u64)
                    .expect("modulo world_size fits usize");
                Ok(BatchRoute {
                    input_batch_id: batch.batch_id,
                    rank: DistributedTokenizerRank::new(target, self.rank.world_size)?,
                    shard_id: None,
                })
            }
            RoutingStrategy::ShardOwnership => self.route_by_shard(batch),
        }
    }

    fn route_by_shard(&self, batch: &RawSampleBatch) -> TokenizerResult<BatchRoute> {
        let shard = self
            .shards
            .iter()
            .find(|shard| shard.contains_offset(batch.batch_id))
            .ok_or_else(|| {
                TokenizerError::InvalidConfig(format!(
                    "no tokenizer shard owns input batch {}",
                    batch.batch_id
                ))
            })?;

        Ok(BatchRoute {
            input_batch_id: batch.batch_id,
            rank: shard.rank,
            shard_id: Some(shard.shard_id),
        })
    }
}
