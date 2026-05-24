//! Tokenizer error model.

use std::{io, path::PathBuf};

use thiserror::Error;
use tunex_core::{error::ErrorKind, TunexError};

/// Result alias for tokenizer operations.
pub type TokenizerResult<T> = Result<T, TokenizerError>;

/// Errors produced by tokenizer modules.
#[derive(Debug, Error)]
pub enum TokenizerError {
    /// Tokenizer model file could not be read.
    #[error("tokenizer I/O failed for {path}: {source}")]
    Io {
        /// Path being read.
        path: PathBuf,
        /// Source error.
        #[source]
        source: io::Error,
    },
    /// Hugging Face tokenizer failed.
    #[error("Hugging Face tokenizer failed: {0}")]
    Hf(String),
    /// Input bytes were not valid UTF-8.
    #[error("sample payload is not valid UTF-8")]
    InvalidUtf8,
    /// Configuration is invalid.
    #[error("invalid tokenizer config: {0}")]
    InvalidConfig(String),
}

impl TokenizerError {
    /// Creates an I/O error with path context.
    #[must_use]
    pub fn io(path: impl Into<PathBuf>, source: io::Error) -> Self {
        Self::Io {
            path: path.into(),
            source,
        }
    }
}

impl From<TokenizerError> for TunexError {
    fn from(error: TokenizerError) -> Self {
        TunexError::new(ErrorKind::Tokenizer, error.to_string()).recoverable()
    }
}
