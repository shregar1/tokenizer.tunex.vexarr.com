//! TuneX tokenizer plugin capabilities.

use tunex_core::plugin::PluginCapability;

/// Hugging Face tokenizer file compatibility.
pub const HF_TOKENIZER: &str = "hf-tokenizer";
/// Batch tokenization support.
pub const BATCH_TOKENIZATION: &str = "batch-tokenization";
/// Parallel batch tokenization support.
pub const PARALLEL_TOKENIZATION: &str = "parallel-tokenization";
/// Chat template formatting support.
pub const CHAT_TEMPLATE: &str = "chat-template";
/// Special token handling support.
pub const SPECIAL_TOKENS: &str = "special-tokens";
/// Sequence truncation support.
pub const TRUNCATION: &str = "truncation";
/// Padding support.
pub const PADDING: &str = "padding";

/// Stable capability list advertised by the tokenizer plugin.
pub const TOKENIZER_CAPABILITY_NAMES: &[&str] = &[
    HF_TOKENIZER,
    BATCH_TOKENIZATION,
    PARALLEL_TOKENIZATION,
    CHAT_TEMPLATE,
    SPECIAL_TOKENS,
    TRUNCATION,
    PADDING,
];

/// Builds core plugin capability values.
#[must_use]
pub fn tokenizer_capabilities() -> Vec<PluginCapability> {
    TOKENIZER_CAPABILITY_NAMES
        .iter()
        .map(|capability| PluginCapability::new(*capability))
        .collect()
}
