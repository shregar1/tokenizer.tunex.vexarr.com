//! Tokenizer cache key construction.

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::config::TokenizerConfig;

/// Stable tokenizer fingerprint used to invalidate stale cache entries.
pub type TokenizerFingerprint = String;

/// Cached artifact category.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CacheTarget {
    /// Raw text token ids.
    RawText,
    /// Formatted chat token ids.
    FormattedChat,
    /// Chat template output.
    TemplateOutput,
    /// Complete tokenized batch.
    TokenizedBatch,
}

/// Complete cache key for tokenizer outputs.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TokenizerCacheKey {
    /// Hash of raw or formatted text bytes.
    pub text_hash: u64,
    /// Tokenizer model/config fingerprint.
    pub tokenizer_fingerprint: TokenizerFingerprint,
    /// Chat template version or name.
    pub chat_template_version: Option<String>,
    /// Maximum sequence length.
    pub max_seq_len: Option<usize>,
    /// Whether truncation is enabled.
    pub truncation: bool,
    /// Whether BOS insertion is enabled.
    pub add_bos: bool,
    /// Whether EOS insertion is enabled.
    pub add_eos: bool,
    /// BOS token id.
    pub bos_token_id: Option<u32>,
    /// EOS token id.
    pub eos_token_id: Option<u32>,
    /// Cached artifact category.
    pub target: CacheTarget,
}

impl TokenizerCacheKey {
    /// Creates a cache key from text bytes and tokenizer config.
    #[must_use]
    pub fn from_text(
        text: impl AsRef<[u8]>,
        tokenizer_fingerprint: impl Into<TokenizerFingerprint>,
        config: &TokenizerConfig,
        target: CacheTarget,
    ) -> Self {
        Self {
            text_hash: hash_bytes(text.as_ref()),
            tokenizer_fingerprint: tokenizer_fingerprint.into(),
            chat_template_version: config.chat_template.clone(),
            max_seq_len: config.max_seq_len,
            truncation: config.truncate,
            add_bos: config.add_bos,
            add_eos: config.add_eos,
            bos_token_id: config.bos_token_id,
            eos_token_id: config.eos_token_id,
            target,
        }
    }

    /// Creates a cache key for a tokenized batch from a set of text hashes.
    #[must_use]
    pub fn from_text_hashes(
        text_hashes: &[u64],
        tokenizer_fingerprint: impl Into<TokenizerFingerprint>,
        config: &TokenizerConfig,
    ) -> Self {
        let mut hasher = DefaultHasher::new();
        text_hashes.hash(&mut hasher);
        Self {
            text_hash: hasher.finish(),
            tokenizer_fingerprint: tokenizer_fingerprint.into(),
            chat_template_version: config.chat_template.clone(),
            max_seq_len: config.max_seq_len,
            truncation: config.truncate,
            add_bos: config.add_bos,
            add_eos: config.add_eos,
            bos_token_id: config.bos_token_id,
            eos_token_id: config.eos_token_id,
            target: CacheTarget::TokenizedBatch,
        }
    }
}

/// Hashes text bytes for cache key construction.
#[must_use]
pub fn hash_bytes(bytes: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();
    bytes.hash(&mut hasher);
    hasher.finish()
}
