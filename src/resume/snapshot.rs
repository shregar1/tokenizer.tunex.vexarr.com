//! Versioned tokenizer resume snapshots.

use crate::TokenizerResult;

use super::{TokenizerCursor, TokenizerResumeState};

/// Versioned tokenizer resume snapshot stored with trainer checkpoints.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenizerResumeSnapshot {
    /// Snapshot format version.
    pub version: u32,
    /// Resume state payload.
    pub state: TokenizerResumeState,
    /// Optional checkpoint id.
    pub checkpoint_id: Option<String>,
    /// Optional trainer step.
    pub checkpoint_step: Option<u64>,
}

impl TokenizerResumeSnapshot {
    /// Current snapshot schema version.
    pub const VERSION: u32 = 1;

    /// Creates a snapshot from resume state.
    pub fn new(state: TokenizerResumeState) -> TokenizerResult<Self> {
        state.validate()?;
        Ok(Self {
            version: Self::VERSION,
            state,
            checkpoint_id: None,
            checkpoint_step: None,
        })
    }

    /// Creates a snapshot directly from a cursor.
    pub fn from_cursor(
        cursor: TokenizerCursor,
        config_fingerprint: impl Into<String>,
    ) -> TokenizerResult<Self> {
        Self::new(TokenizerResumeState::new(cursor, config_fingerprint))
    }

    /// Attaches checkpoint identity.
    #[must_use]
    pub fn with_checkpoint(mut self, checkpoint_id: impl Into<String>, step: u64) -> Self {
        self.checkpoint_id = Some(checkpoint_id.into());
        self.checkpoint_step = Some(step);
        self
    }

    /// Returns the embedded cursor.
    #[must_use]
    pub const fn cursor(&self) -> &TokenizerCursor {
        &self.state.cursor
    }
}
