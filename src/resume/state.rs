//! Tokenizer resume state.

use crate::{TokenizerError, TokenizerResult};

use super::{ResumeValidationReport, TokenizerCursor};

/// Complete tokenizer resume state required for deterministic replay.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenizerResumeState {
    /// Exact tokenizer cursor at checkpoint time.
    pub cursor: TokenizerCursor,
    /// Config fingerprint used to create the tokenizer.
    pub config_fingerprint: String,
    /// Cache namespace fingerprint.
    pub cache_fingerprint: Option<String>,
    /// Distributed world size, when sharded.
    pub world_size: Option<u32>,
    /// Last emitted distributed batch id.
    pub last_distributed_batch_id: Option<u64>,
}

impl TokenizerResumeState {
    /// Creates resume state from cursor.
    #[must_use]
    pub fn new(cursor: TokenizerCursor, config_fingerprint: impl Into<String>) -> Self {
        Self {
            cursor,
            config_fingerprint: config_fingerprint.into(),
            cache_fingerprint: None,
            world_size: None,
            last_distributed_batch_id: None,
        }
    }

    /// Adds cache fingerprint metadata.
    #[must_use]
    pub fn with_cache_fingerprint(mut self, cache_fingerprint: impl Into<String>) -> Self {
        self.cache_fingerprint = Some(cache_fingerprint.into());
        self
    }

    /// Adds distributed replay metadata.
    #[must_use]
    pub const fn with_distributed(
        mut self,
        world_size: u32,
        last_distributed_batch_id: Option<u64>,
    ) -> Self {
        self.world_size = Some(world_size);
        self.last_distributed_batch_id = last_distributed_batch_id;
        self
    }

    /// Validates internal consistency.
    pub fn validate(&self) -> TokenizerResult<()> {
        let report = self.validation_report();
        if report.is_valid() {
            return Ok(());
        }

        Err(TokenizerError::InvalidConfig(report.to_string()))
    }

    /// Returns a validation report.
    #[must_use]
    pub fn validation_report(&self) -> ResumeValidationReport {
        let mut report = ResumeValidationReport::default();

        if self.cursor.tokenizer_fingerprint.is_empty() {
            report.push(
                "cursor.tokenizer_fingerprint",
                "tokenizer fingerprint is required",
            );
        }
        if self.cursor.template_fingerprint.is_empty() {
            report.push(
                "cursor.template_fingerprint",
                "template fingerprint is required",
            );
        }
        if self.config_fingerprint.is_empty() {
            report.push("config_fingerprint", "config fingerprint is required");
        }
        if let Some(world_size) = self.world_size {
            if world_size == 0 {
                report.push("world_size", "world_size must be greater than zero");
            }
            match self.cursor.rank {
                Some(rank) if rank < world_size => {}
                Some(rank) => report.push(
                    "cursor.rank",
                    format!("rank {rank} must be less than world_size {world_size}"),
                ),
                None => report.push(
                    "cursor.rank",
                    "rank is required when world_size is configured",
                ),
            }
        }
        if self.cursor.rank.is_some() && self.cursor.shard_id.is_none() {
            report.push(
                "cursor.shard_id",
                "shard_id is required when rank is configured",
            );
        }

        report
    }
}
