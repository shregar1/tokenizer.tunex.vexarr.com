//! Top-level tokenizer configuration.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::{PaddingConfig, TemplateConfig, TruncationConfig};

/// Hugging Face compatible tokenizer configuration.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(default)]
pub struct TokenizerConfig {
    /// Tokenizer plugin id.
    pub plugin: String,
    /// Optional tokenizer JSON path.
    pub tokenizer_path: Option<PathBuf>,
    /// Optional remote or registry tokenizer name.
    pub tokenizer_name: Option<String>,
    /// Maximum sequence length to keep.
    pub max_seq_len: Option<usize>,
    /// Whether to truncate sequences longer than `max_seq_len`.
    #[serde(alias = "truncation")]
    pub truncate: bool,
    /// Whether to include tokenizer-level special tokens.
    pub add_special_tokens: bool,
    /// Whether to pad batch encodings.
    pub padding: bool,
    /// Optional fixed pad length. Batch-longest padding is used when omitted.
    pub pad_to_length: Option<usize>,
    /// Padding token id.
    pub pad_token_id: u32,
    /// Padding token string.
    pub pad_token: String,
    /// Optional BOS token id inserted after HF encoding.
    pub bos_token_id: Option<u32>,
    /// Optional EOS token id appended after HF encoding.
    pub eos_token_id: Option<u32>,
    /// Chat template name, such as `llama3`, `alpaca`, `sharegpt`, or `openai`.
    pub chat_template: Option<String>,
    /// Whether text formatting should add a BOS marker.
    pub add_bos: bool,
    /// Whether text formatting should add an EOS marker.
    pub add_eos: bool,
    /// Number of Rayon workers intended for CPU tokenization.
    pub workers: usize,
    /// Target batch size for batch tokenization.
    pub batch_size: usize,
    /// Whether parallel batch tokenization is enabled.
    pub parallel: bool,
}

impl TokenizerConfig {
    /// Creates config from a tokenizer JSON path.
    #[must_use]
    pub fn from_path(path: impl Into<PathBuf>) -> Self {
        Self {
            tokenizer_path: Some(path.into()),
            ..Self::default()
        }
    }

    /// Returns padding config.
    #[must_use]
    pub fn padding_config(&self) -> PaddingConfig {
        PaddingConfig {
            enabled: self.padding,
            pad_to_length: self.pad_to_length,
            pad_token_id: self.pad_token_id,
            pad_token: self.pad_token.clone(),
        }
    }

    /// Returns truncation config.
    #[must_use]
    pub const fn truncation_config(&self) -> TruncationConfig {
        TruncationConfig {
            enabled: self.truncate,
            max_seq_len: self.max_seq_len,
        }
    }

    /// Returns template config.
    #[must_use]
    pub fn template_config(&self) -> TemplateConfig {
        TemplateConfig {
            chat_template: self.chat_template.clone(),
            add_bos: self.add_bos,
            add_eos: self.add_eos,
            bos_token_id: self.bos_token_id,
            eos_token_id: self.eos_token_id,
        }
    }
}

impl Default for TokenizerConfig {
    fn default() -> Self {
        Self {
            plugin: "tunex-tokenizer-hf".to_owned(),
            tokenizer_path: None,
            tokenizer_name: None,
            max_seq_len: None,
            truncate: true,
            add_special_tokens: true,
            padding: false,
            pad_to_length: None,
            pad_token_id: 0,
            pad_token: "[PAD]".to_owned(),
            bos_token_id: None,
            eos_token_id: None,
            chat_template: None,
            add_bos: false,
            add_eos: false,
            workers: std::thread::available_parallelism().map_or(1, usize::from),
            batch_size: 1024,
            parallel: true,
        }
    }
}
