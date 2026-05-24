//! Deterministic replay verification.

use tunex_core::types::RawSample;

use crate::{tokenizer::TunexTokenizer, TokenizedBatch, TokenizerError, TokenizerResult};

use super::TokenizerResumeState;

/// Replay plan derived from a resume state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayPlan {
    /// Resume state to replay from.
    pub state: TokenizerResumeState,
    /// Number of samples expected to be skipped before producing continuation.
    pub skip_samples: u64,
    /// Number of batches expected to be skipped before producing continuation.
    pub skip_batches: u64,
}

impl ReplayPlan {
    /// Creates a replay plan.
    pub fn new(state: TokenizerResumeState) -> TokenizerResult<Self> {
        state.validate()?;
        Ok(Self {
            skip_samples: state.cursor.sample_index,
            skip_batches: state.cursor.batch_index,
            state,
        })
    }
}

/// Replay verification result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayResult {
    /// Batch produced during replay.
    pub batch: TokenizedBatch,
    /// Whether replay output matched expected output.
    pub matched: bool,
}

/// Verifies deterministic replay for tokenizer outputs.
#[derive(Clone, Debug)]
pub struct ReplayVerifier {
    plan: ReplayPlan,
}

impl ReplayVerifier {
    /// Creates a replay verifier.
    #[must_use]
    pub const fn new(plan: ReplayPlan) -> Self {
        Self { plan }
    }

    /// Returns replay plan.
    #[must_use]
    pub const fn plan(&self) -> &ReplayPlan {
        &self.plan
    }

    /// Tokenizes a continuation batch and compares it to an expected batch.
    ///
    /// # Errors
    ///
    /// Returns an error if tokenization fails or replay fingerprints mismatch.
    pub fn verify_batch<T>(
        &self,
        tokenizer: &T,
        samples: &[RawSample],
        expected: &TokenizedBatch,
        tokenizer_fingerprint: &str,
        template_fingerprint: &str,
    ) -> TokenizerResult<ReplayResult>
    where
        T: TunexTokenizer,
    {
        self.validate_fingerprints(tokenizer_fingerprint, template_fingerprint)?;
        let batch = tokenizer.tokenize_batch(samples)?;
        let matched = same_tokenized_content(&batch, expected);
        Ok(ReplayResult { batch, matched })
    }

    fn validate_fingerprints(
        &self,
        tokenizer_fingerprint: &str,
        template_fingerprint: &str,
    ) -> TokenizerResult<()> {
        let cursor = &self.plan.state.cursor;
        if cursor.tokenizer_fingerprint != tokenizer_fingerprint {
            return Err(TokenizerError::InvalidConfig(format!(
                "tokenizer fingerprint mismatch: expected {}, got {}",
                cursor.tokenizer_fingerprint, tokenizer_fingerprint
            )));
        }
        if cursor.template_fingerprint != template_fingerprint {
            return Err(TokenizerError::InvalidConfig(format!(
                "template fingerprint mismatch: expected {}, got {}",
                cursor.template_fingerprint, template_fingerprint
            )));
        }
        Ok(())
    }
}

fn same_tokenized_content(left: &TokenizedBatch, right: &TokenizedBatch) -> bool {
    left.input_ids == right.input_ids
        && left.offsets == right.offsets
        && left.lengths == right.lengths
        && left.attention_mask == right.attention_mask
        && left.sample_ids == right.sample_ids
        && left.metadata.samples == right.metadata.samples
        && left.metadata.tokens == right.metadata.tokens
}
