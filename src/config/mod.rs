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
            batch_size: 1024,
            workers: std::thread::available_parallelism().map_or(1, usize::from),
        }
    }
}
