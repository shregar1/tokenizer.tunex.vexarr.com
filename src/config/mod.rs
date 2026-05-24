//! Tokenizer configuration.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Hugging Face compatible tokenizer configuration.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(default)]
pub struct TokenizerConfig {
    /// Optional tokenizer JSON path.
    pub tokenizer_path: Option<PathBuf>,
    /// Maximum sequence length to keep.
    pub max_seq_len: Option<usize>,
    /// Whether to truncate sequences longer than `max_seq_len`.
    pub truncate: bool,
    /// Whether to include special tokens.
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
    /// Target batch size for batch tokenization.
    pub batch_size: usize,
    /// Number of Rayon workers intended for CPU tokenization.
    pub workers: usize,
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
}

impl Default for TokenizerConfig {
    fn default() -> Self {
        Self {
            tokenizer_path: None,
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
            batch_size: 1024,
            workers: std::thread::available_parallelism().map_or(1, usize::from),
        }
    }
}
