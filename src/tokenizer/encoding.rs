//! Hugging Face encoding conversion helpers.

use tokenizers::Encoding;
use tunex_core::types::TokenizedSample;

use super::SpecialTokenConfig;

/// Converts Hugging Face encodings into TuneX core samples.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EncodingAdapter {
    special_tokens: SpecialTokenConfig,
    max_seq_len: Option<usize>,
    truncate: bool,
}

impl EncodingAdapter {
    /// Creates a new encoding adapter.
    #[must_use]
    pub const fn new(
        special_tokens: SpecialTokenConfig,
        max_seq_len: Option<usize>,
        truncate: bool,
    ) -> Self {
        Self {
            special_tokens,
            max_seq_len,
            truncate,
        }
    }

    /// Converts one encoding.
    #[must_use]
    pub fn convert(&self, id: Option<String>, encoding: &Encoding) -> TokenizedSample {
        let mut token_ids = encoding.get_ids().to_vec();
        let mut attention_mask = encoding
            .get_attention_mask()
            .iter()
            .map(|value| u8::from(*value > 0))
            .collect::<Vec<_>>();

        self.special_tokens
            .apply(&mut token_ids, &mut attention_mask);

        if let Some(max_seq_len) = self.max_seq_len {
            if self.truncate && token_ids.len() > max_seq_len {
                token_ids.truncate(max_seq_len);
                attention_mask.truncate(max_seq_len);
            }
        }

        TokenizedSample {
            id,
            token_ids,
            attention_mask,
        }
    }
}
