//! Distributed tokenizer synchronization hooks.

use crate::TokenizerResult;

use super::{DistributedTokenizerEpoch, DistributedTokenizerRank};

/// Synchronization checkpoint.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SyncCheckpoint {
    /// Rank reaching the checkpoint.
    pub rank: DistributedTokenizerRank,
    /// Current epoch.
    pub epoch: DistributedTokenizerEpoch,
    /// Last completed distributed batch id.
    pub last_batch_id: Option<u64>,
}

/// Barrier result shared by distributed workers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SyncBarrier {
    /// Synchronized epoch.
    pub epoch: DistributedTokenizerEpoch,
    /// Workers acknowledged at the barrier.
    pub acknowledged_workers: usize,
}

/// Hook for future distributed synchronization backends.
pub trait SyncHook: Send + Sync {
    /// Called when a tokenizer worker reaches a checkpoint.
    ///
    /// # Errors
    ///
    /// Returns an error when synchronization fails.
    fn synchronize(&self, checkpoint: SyncCheckpoint) -> TokenizerResult<SyncBarrier>;
}

impl<F> SyncHook for F
where
    F: Fn(SyncCheckpoint) -> TokenizerResult<SyncBarrier> + Send + Sync,
{
    fn synchronize(&self, checkpoint: SyncCheckpoint) -> TokenizerResult<SyncBarrier> {
        self(checkpoint)
    }
}
