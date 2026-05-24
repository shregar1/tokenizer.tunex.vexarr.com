use tunex_tokenizer::prelude::*;

fn main() -> Result<(), TemplateValidationError> {
    let formatter = TemplateFormatter::new(
        ChatTemplate::new(ChatTemplateConfig::new(TemplateKind::Llama3)),
        TextSpecialTokens {
            add_bos: true,
            add_eos: true,
            ..TextSpecialTokens::default()
        },
    );
    let messages = vec![
        ChatMessage::system("You are concise."),
        ChatMessage::user("Explain TuneX tokenizer contracts."),
    ];
    let formatted = formatter.format(&messages)?;

    println!("{formatted}");
    Ok(())
}
