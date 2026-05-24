use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use tunex_tokenizer::{
    ChatMessage, ChatTemplate, ChatTemplateConfig, TemplateFormatter, TemplateKind,
    TextSpecialTokens,
};

const FORMAT_COUNT: usize = 8192;

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
    let no_template = "Summarize tokenizer benchmark goals.";
    let mut group = c.benchmark_group("template_format");
    group.throughput(Throughput::Elements(FORMAT_COUNT as u64));

    group.bench_function("template_overhead", |b| {
        b.iter(|| {
            let mut bytes = 0usize;
            for _ in 0..FORMAT_COUNT {
                bytes += formatter.format(&input).expect("formatted").len();
            }
            black_box(bytes);
        });
    });

    group.bench_function("no_template_baseline", |b| {
        b.iter(|| {
            let mut bytes = 0usize;
            for _ in 0..FORMAT_COUNT {
                bytes += no_template.len();
            }
            black_box(bytes);
        });
    });

    group.finish();
}

criterion_group!(benches, bench_template_format);
criterion_main!(benches);
