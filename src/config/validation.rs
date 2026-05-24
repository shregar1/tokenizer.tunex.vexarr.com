//! Tokenizer config validation.

use super::{ChatTemplateSelection, TokenizerConfig};

/// One config validation error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigValidationError {
    /// Dot path of the invalid config value.
    pub path: String,
    /// Human-readable validation message.
    pub message: String,
}

impl ConfigValidationError {
    /// Creates a validation error.
    #[must_use]
    pub fn new(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            message: message.into(),
        }
    }
}

/// Tokenizer config validation report.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ConfigValidationReport {
    /// Validation errors.
    pub errors: Vec<ConfigValidationError>,
}

impl ConfigValidationReport {
    /// Returns true when no validation errors exist.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    fn push(&mut self, path: impl Into<String>, message: impl Into<String>) {
        self.errors.push(ConfigValidationError::new(path, message));
    }
}

/// Validation contract for tokenizer configs.
pub trait ValidateTokenizerConfig {
    /// Validates this config.
    fn validate(&self) -> ConfigValidationReport;
}

impl ValidateTokenizerConfig for TokenizerConfig {
    fn validate(&self) -> ConfigValidationReport {
        let mut report = ConfigValidationReport::default();

        if let Some(path) = &self.tokenizer_path {
            if !path.exists() {
                report.push("tokenizer_path", "tokenizer file does not exist");
            }
        }
        if self.tokenizer_path.is_none() && self.tokenizer_name.is_none() {
            report.push(
                "tokenizer_path",
                "tokenizer_path or tokenizer_name must be provided",
            );
        }
        if self.max_seq_len.is_some_and(|value| value == 0) {
            report.push("max_seq_len", "max_seq_len must be greater than zero");
        }
        if self.workers == 0 {
            report.push("workers", "workers must be greater than zero");
        }
        if self.batch_size == 0 {
            report.push("batch_size", "batch_size must be greater than zero");
        }
        if let Some(template) = &self.chat_template {
            if ChatTemplateSelection::parse(template).is_none() {
                report.push("chat_template", "unknown chat template");
            }
        }
        if self.add_bos && self.bos_token_id.is_none() {
            report.push("bos_token_id", "bos_token_id is required when add_bos=true");
        }
        if self.add_eos && self.eos_token_id.is_none() {
            report.push("eos_token_id", "eos_token_id is required when add_eos=true");
        }
        if self.padding && self.pad_to_length.is_some_and(|pad| pad == 0) {
            report.push("pad_to_length", "pad_to_length must be greater than zero");
        }
        if self.padding
            && self.truncate
            && matches!(
                (self.pad_to_length, self.max_seq_len),
                (Some(pad), Some(max)) if pad < max
            )
        {
            report.push(
                "padding",
                "pad_to_length must be >= max_seq_len when padding and truncation are enabled",
            );
        }

        report
    }
}
