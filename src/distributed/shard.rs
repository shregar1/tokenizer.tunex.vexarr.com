//! Distributed tokenizer shard descriptors.

use crate::{TokenizerError, TokenizerResult};

/// Identifies one tokenizer worker in a distributed world.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DistributedTokenizerRank {
    /// Current rank.
    pub rank: usize,
    /// Total participating ranks.
    pub world_size: usize,
}

impl DistributedTokenizerRank {
    /// Creates a validated distributed rank.
    pub fn new(rank: usize, world_size: usize) -> TokenizerResult<Self> {
        let descriptor = Self { rank, world_size };
        descriptor.validate()?;
        Ok(descriptor)
    }

    /// Validates rank membership.
    pub fn validate(self) -> TokenizerResult<()> {
        if self.world_size == 0 {
            return Err(TokenizerError::InvalidConfig(
                "distributed tokenizer world_size must be greater than zero".to_owned(),
            ));
        }
        if self.rank >= self.world_size {
            return Err(TokenizerError::InvalidConfig(format!(
                "distributed tokenizer rank {} must be less than world_size {}",
                self.rank, self.world_size
            )));
        }
        Ok(())
    }
}

/// Synchronized distributed tokenizer epoch.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DistributedTokenizerEpoch {
    /// Current epoch.
    pub epoch: u64,
    /// Deterministic seed shared by all ranks.
    pub seed: u64,
}

impl DistributedTokenizerEpoch {
    /// Creates epoch state.
    #[must_use]
    pub const fn new(epoch: u64, seed: u64) -> Self {
        Self { epoch, seed }
    }

    /// Returns the next epoch.
    #[must_use]
    pub const fn next(self) -> Self {
        Self {
            epoch: self.epoch + 1,
            seed: self.seed,
        }
    }
}

impl Default for DistributedTokenizerEpoch {
    fn default() -> Self {
        Self { epoch: 0, seed: 42 }
    }
}

/// Dataset shard ownership assigned to one tokenizer rank.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenizerShard {
    /// Owning tokenizer rank.
    pub rank: DistributedTokenizerRank,
    /// Synchronized epoch.
    pub epoch: DistributedTokenizerEpoch,
    /// Dataset shard id.
    pub shard_id: u64,
    /// First sample offset in shard.
    pub start_offset: u64,
    /// Number of samples in shard.
    pub len: u64,
}

impl TokenizerShard {
    /// Creates shard ownership metadata.
    #[must_use]
    pub const fn new(
        rank: DistributedTokenizerRank,
        epoch: DistributedTokenizerEpoch,
        shard_id: u64,
        start_offset: u64,
        len: u64,
    ) -> Self {
        Self {
            rank,
            epoch,
            shard_id,
            start_offset,
            len,
        }
    }

    /// Returns true when the global sample offset belongs to this shard.
    #[must_use]
    pub const fn contains_offset(&self, offset: u64) -> bool {
        offset >= self.start_offset && offset < self.start_offset + self.len
    }
}
