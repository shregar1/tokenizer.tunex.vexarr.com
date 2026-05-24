//! Truncation configuration.

use serde::{Deserialize, Serialize};

/// Tokenizer truncation configuration.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(default)]
pub struct TruncationConfig {
    /// Whether truncation is enabled.
    pub enabled: bool,
    /// Maximum sequence length to keep.
    pub max_seq_len: Option<usize>,
}

impl Default for TruncationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_seq_len: None,
        }
    }
}
