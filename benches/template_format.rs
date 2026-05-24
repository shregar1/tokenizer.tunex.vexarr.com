use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use tunex_tokenizer::{
    ChatMessage, ChatTemplate, ChatTemplateConfig, TemplateFormatter, TemplateKind,
    TextSpecialTokens,
};

fn messages() -> Vec<ChatMessage> {
    vec![
        ChatMessage::system("You are a concise assistant."),
        ChatMessage::user("Summarize tokenizer benchmark goals."),
        ChatMessage::assistant("Measure latency, throughput, and allocation pressure."),
    ]
}

fn bench_template_format(c: &mut Criterion) {
    let formatter = TemplateFormatter::new(
        ChatTemplate::new(ChatTemplateConfig::new(TemplateKind::Llama3)),
        TextSpecialTokens::default(),
    );
    let input = messages();
    let mut group = c.benchmark_group("template_format");
    group.throughput(Throughput::Elements(input.len() as u64));
    group.bench_function("llama3", |b| {
        b.iter(|| {
            let formatted = formatter.format(&input).expect("formatted");
            black_box(formatted.len());
        });
    });
    group.finish();
}

criterion_group!(benches, bench_template_format);
criterion_main!(benches);
