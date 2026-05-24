//! Chat template configuration.

use serde::{Deserialize, Serialize};

/// Supported template selection.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatTemplateSelection {
    /// Llama 3 style role headers.
    Llama3,
    /// Alpaca instruction tuning format.
    Alpaca,
    /// ShareGPT style conversations.
    ShareGpt,
    /// OpenAI messages style.
    OpenAi,
    /// Custom template string.
    Custom(String),
}

impl ChatTemplateSelection {
    /// Parses a config string.
    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "llama3" | "llama-3" => Some(Self::Llama3),
            "alpaca" => Some(Self::Alpaca),
            "sharegpt" | "share_gpt" => Some(Self::ShareGpt),
            "openai" | "openai_messages" => Some(Self::OpenAi),
            custom if custom.starts_with("custom:") => Some(Self::Custom(value[7..].to_owned())),
            _ => None,
        }
    }
}

/// Chat template and text special-token configuration.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(default)]
pub struct TemplateConfig {
    /// Template name or custom template selector.
    pub chat_template: Option<String>,
    /// Whether text formatting should add a BOS marker.
    pub add_bos: bool,
    /// Whether text formatting should add an EOS marker.
    pub add_eos: bool,
    /// Optional BOS token id used by token-id special handling.
    pub bos_token_id: Option<u32>,
    /// Optional EOS token id used by token-id special handling.
    pub eos_token_id: Option<u32>,
}
