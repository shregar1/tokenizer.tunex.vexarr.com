//! Chat template validation.

use thiserror::Error;

/// Template validation errors.
#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum TemplateValidationError {
    /// No messages were provided.
    #[error("chat template requires at least one message")]
    EmptyMessages,
    /// Required user message is missing.
    #[error("chat template requires at least one user message")]
    MissingUser,
    /// Message content is empty.
    #[error("chat message at index {index} is empty")]
    EmptyContent {
        /// Message index.
        index: usize,
    },
    /// Formatted text exceeds configured max length.
    #[error("formatted template length {actual} exceeds max length {max}")]
    MaxLengthExceeded {
        /// Actual character count.
        actual: usize,
        /// Maximum allowed character count.
        max: usize,
    },
}
