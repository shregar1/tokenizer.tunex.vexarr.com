use tunex_tokenizer::{
    ChatMessage, ChatTemplate, ChatTemplateConfig, TemplateFormatter, TemplateKind,
    TemplateValidationError, TextSpecialTokens,
};

#[test]
fn llama3_template_formats_roles_deterministically() {
    let formatter = TemplateFormatter::new(
        ChatTemplate::new(ChatTemplateConfig::new(TemplateKind::Llama3)),
        TextSpecialTokens::default(),
    );
    let messages = vec![
        ChatMessage::system("be precise"),
        ChatMessage::user("hello"),
        ChatMessage::assistant("hi"),
    ];

    let left = formatter.format(&messages).expect("format");
    let right = formatter.format(&messages).expect("format");

    assert_eq!(left, right);
    assert!(left.contains("<|start_header_id|>user<|end_header_id|>"));
    assert!(left.contains("<|eot_id|>"));
}

#[test]
fn alpaca_template_formats_instruction_response() {
    let formatter = TemplateFormatter::new(
        ChatTemplate::new(ChatTemplateConfig::new(TemplateKind::Alpaca)),
        TextSpecialTokens::default(),
    );

    let output = formatter
        .format(&[
            ChatMessage::user("Summarize this."),
            ChatMessage::assistant("Short summary."),
        ])
        .expect("format");

    assert!(output.contains("### Instruction:"));
    assert!(output.contains("### Response:"));
}

#[test]
fn custom_template_supports_role_and_content_placeholders() {
    let formatter = TemplateFormatter::new(
        ChatTemplate::new(ChatTemplateConfig::new(TemplateKind::Custom(
            "<{{role}}>{{content}}\n".to_owned(),
        ))),
        TextSpecialTokens::default(),
    );

    let output = formatter
        .format(&[ChatMessage::user("question")])
        .expect("format");

    assert_eq!(output, "<user>question\n");
}

#[test]
fn validation_detects_missing_user_message() {
    let formatter = TemplateFormatter::new(
        ChatTemplate::new(ChatTemplateConfig::new(TemplateKind::OpenAi)),
        TextSpecialTokens::default(),
    );

    let error = formatter
        .format(&[ChatMessage::assistant("hello")])
        .expect_err("missing user");

    assert_eq!(error, TemplateValidationError::MissingUser);
}

#[test]
fn validation_enforces_max_length_after_special_tokens() {
    let formatter = TemplateFormatter::new(
        ChatTemplate::new(ChatTemplateConfig::new(TemplateKind::ShareGpt).with_max_chars(4)),
        TextSpecialTokens {
            add_bos: true,
            add_eos: true,
            ..TextSpecialTokens::default()
        },
    );

    let error = formatter
        .format(&[ChatMessage::user("hello")])
        .expect_err("too long");

    assert!(matches!(
        error,
        TemplateValidationError::MaxLengthExceeded { .. }
    ));
}
