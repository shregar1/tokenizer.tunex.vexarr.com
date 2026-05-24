//! Deterministic chat template formatting.

use super::{
    ChatMessage, ChatRole, ChatTemplate, TemplateKind, TemplateValidationError, TextSpecialTokens,
};

/// Chat template formatter.
#[derive(Clone, Debug)]
pub struct TemplateFormatter {
    template: ChatTemplate,
    special_tokens: TextSpecialTokens,
}

impl TemplateFormatter {
    /// Creates a formatter.
    #[must_use]
    pub const fn new(template: ChatTemplate, special_tokens: TextSpecialTokens) -> Self {
        Self {
            template,
            special_tokens,
        }
    }

    /// Formats chat messages.
    pub fn format(&self, messages: &[ChatMessage]) -> Result<String, TemplateValidationError> {
        self.validate_messages(messages)?;
        let body = match &self.template.config.kind {
            TemplateKind::Llama3 => format_llama3(messages),
            TemplateKind::Alpaca => format_alpaca(messages),
            TemplateKind::ShareGpt => format_sharegpt(messages),
            TemplateKind::OpenAi => format_openai(messages),
            TemplateKind::Custom(template) => format_custom(messages, template),
        };
        let output = self.special_tokens.apply(body);
        self.validate_length(&output)?;
        Ok(output)
    }

    fn validate_messages(&self, messages: &[ChatMessage]) -> Result<(), TemplateValidationError> {
        if messages.is_empty() {
            return Err(TemplateValidationError::EmptyMessages);
        }
        if self.template.config.require_user
            && !messages
                .iter()
                .any(|message| message.role == ChatRole::User)
        {
            return Err(TemplateValidationError::MissingUser);
        }
        for (index, message) in messages.iter().enumerate() {
            if message.content.trim().is_empty() {
                return Err(TemplateValidationError::EmptyContent { index });
            }
        }
        Ok(())
    }

    fn validate_length(&self, output: &str) -> Result<(), TemplateValidationError> {
        let Some(max) = self.template.config.max_chars else {
            return Ok(());
        };
        let actual = output.chars().count();
        if actual > max {
            return Err(TemplateValidationError::MaxLengthExceeded { actual, max });
        }
        Ok(())
    }
}

fn format_llama3(messages: &[ChatMessage]) -> String {
    let mut output = String::new();
    for message in messages {
        output.push_str("<|start_header_id|>");
        output.push_str(role_name(&message.role));
        output.push_str("<|end_header_id|>\n\n");
        output.push_str(&message.content);
        output.push_str("<|eot_id|>");
    }
    output
}

fn format_alpaca(messages: &[ChatMessage]) -> String {
    let instruction = messages
        .iter()
        .find(|message| message.role == ChatRole::User)
        .map_or("", |message| message.content.as_str());
    let response = messages
        .iter()
        .rev()
        .find(|message| message.role == ChatRole::Assistant)
        .map_or("", |message| message.content.as_str());
    format!("### Instruction:\n{instruction}\n\n### Response:\n{response}")
}

fn format_sharegpt(messages: &[ChatMessage]) -> String {
    messages
        .iter()
        .map(|message| format!("{}: {}", role_name(&message.role), message.content))
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_openai(messages: &[ChatMessage]) -> String {
    messages
        .iter()
        .map(|message| format!("[{}]\n{}", role_name(&message.role), message.content))
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_custom(messages: &[ChatMessage], template: &str) -> String {
    let mut output = String::new();
    for message in messages {
        let rendered = template
            .replace("{{role}}", role_name(&message.role))
            .replace("{{content}}", &message.content);
        output.push_str(&rendered);
    }
    output
}

fn role_name(role: &ChatRole) -> &'static str {
    match role {
        ChatRole::System => "system",
        ChatRole::User => "user",
        ChatRole::Assistant => "assistant",
    }
}
