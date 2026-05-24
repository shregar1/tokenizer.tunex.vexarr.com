//! Tokenization output helpers.

use tunex_core::types::TokenizedSample;

/// Tokenized sample plus tokenizer-specific metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenizedOutput {
    /// Core tokenized sample.
    pub sample: TokenizedSample,
    /// Token type ids aligned with token ids when available.
    pub token_type_ids: Vec<u32>,
    /// Sequence length after truncation.
    pub sequence_len: usize,
}

impl TokenizedOutput {
    /// Creates output from a core sample and optional token type ids.
    #[must_use]
    pub fn new(sample: TokenizedSample, token_type_ids: Vec<u32>) -> Self {
        let sequence_len = sample.token_ids.len();
        Self {
            sample,
            token_type_ids,
            sequence_len,
        }
    }
}
