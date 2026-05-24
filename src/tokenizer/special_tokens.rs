//! Special token handling.

use smallvec::SmallVec;

/// BOS/EOS token configuration.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SpecialTokenConfig {
    /// Optional BOS token id.
    pub bos_token_id: Option<u32>,
    /// Optional EOS token id.
    pub eos_token_id: Option<u32>,
}

impl SpecialTokenConfig {
    /// Creates special token configuration.
    #[must_use]
    pub const fn new(bos_token_id: Option<u32>, eos_token_id: Option<u32>) -> Self {
        Self {
            bos_token_id,
            eos_token_id,
        }
    }

    /// Applies special tokens to the token ids and attention mask.
    pub fn apply(&self, token_ids: &mut Vec<u32>, attention_mask: &mut Vec<u8>) {
        let mut prefix = SmallVec::<[u32; 2]>::new();
        if let Some(bos) = self.bos_token_id {
            prefix.push(bos);
        }

        if !prefix.is_empty() {
            let mut with_prefix = Vec::with_capacity(prefix.len() + token_ids.len());
            with_prefix.extend(prefix);
            with_prefix.extend(token_ids.iter().copied());
            *token_ids = with_prefix;
            let mut mask = Vec::with_capacity(attention_mask.len() + 1);
            mask.push(1);
            mask.extend(attention_mask.iter().copied());
            *attention_mask = mask;
        }

        if let Some(eos) = self.eos_token_id {
            token_ids.push(eos);
            attention_mask.push(1);
        }
    }
}
