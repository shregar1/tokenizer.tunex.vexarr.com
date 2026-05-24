//! Chat message and template definitions.

/// Chat message role.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ChatRole {
    /// System instruction.
    System,
    /// User message.
    User,
    /// Assistant response.
    Assistant,
}

/// One chat message.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChatMessage {
    /// Message role.
    pub role: ChatRole,
    /// Message content.
    pub content: String,
}

impl ChatMessage {
    /// Creates a system message.
    #[must_use]
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: ChatRole::System,
            content: content.into(),
        }
    }

    /// Creates a user message.
    #[must_use]
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: ChatRole::User,
            content: content.into(),
        }
    }

    /// Creates an assistant message.
    #[must_use]
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: ChatRole::Assistant,
            content: content.into(),
        }
    }
}

/// Supported chat template kinds.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TemplateKind {
    /// Llama 3 style role headers.
    Llama3,
    /// Alpaca instruction tuning format.
    Alpaca,
    /// ShareGPT role format.
    ShareGpt,
    /// OpenAI messages format.
    OpenAi,
    /// Custom format with placeholders.
    Custom(String),
}

impl TemplateKind {
    /// Parses a config template name.
    #[must_use]
    pub fn from_name(name: &str) -> Self {
        match name.to_ascii_lowercase().as_str() {
            "llama3" | "llama-3" => Self::Llama3,
            "alpaca" => Self::Alpaca,
            "sharegpt" | "share_gpt" => Self::ShareGpt,
            "openai" | "openai_messages" => Self::OpenAi,
            _ => Self::Custom(name.to_owned()),
        }
    }
}

/// Chat template config.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChatTemplateConfig {
    /// Template kind.
    pub kind: TemplateKind,
    /// Whether a user message is required.
    pub require_user: bool,
    /// Maximum formatted character length.
    pub max_chars: Option<usize>,
}

impl ChatTemplateConfig {
    /// Creates a config from kind.
    #[must_use]
    pub const fn new(kind: TemplateKind) -> Self {
        Self {
            kind,
            require_user: true,
            max_chars: None,
        }
    }

    /// Sets maximum formatted characters.
    #[must_use]
    pub const fn with_max_chars(mut self, max_chars: usize) -> Self {
        self.max_chars = Some(max_chars);
        self
    }
}

/// Ready-to-use chat template.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChatTemplate {
    /// Template config.
    pub config: ChatTemplateConfig,
}

impl ChatTemplate {
    /// Creates a chat template.
    #[must_use]
    pub const fn new(config: ChatTemplateConfig) -> Self {
        Self { config }
    }
}
