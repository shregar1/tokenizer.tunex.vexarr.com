//! Padding configuration.

use serde::{Deserialize, Serialize};

/// Tokenizer padding configuration.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(default)]
pub struct PaddingConfig {
    /// Whether padding is enabled.
    pub enabled: bool,
    /// Optional fixed pad length. Batch-longest padding is used when omitted.
    pub pad_to_length: Option<usize>,
    /// Padding token id.
    pub pad_token_id: u32,
    /// Padding token string.
    pub pad_token: String,
}

impl Default for PaddingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            pad_to_length: None,
            pad_token_id: 0,
            pad_token: "[PAD]".to_owned(),
        }
    }
}
