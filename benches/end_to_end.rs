use std::hint::black_box;

use bytes::Bytes;
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use tunex_core::types::{RawSample, TokenizedSample};
use tunex_tokenizer::{
    CacheTarget, ChatMessage, ChatTemplate, ChatTemplateConfig, MemoryTokenizerCache,
    RawSampleBatch, TemplateFormatter, TemplateKind, TextSpecialTokens, TokenizedBatch,
    TokenizerCacheEntry, TokenizerCacheKey, TokenizerConfig, TokenizerResult,
    TokenizerStreamAdapter, TunexTokenizer,
};

const SAMPLE_COUNT: usize = 2048;

#[derive(Clone, Copy)]
struct EndToEndTokenizer;

impl TunexTokenizer for EndToEndTokenizer {
    fn tokenize_sample(&self, sample: &RawSample) -> TokenizerResult<TokenizedSample> {
        Ok(TokenizedSample::new(
            sample.payload.iter().map(|byte| u32::from(*byte)).collect(),
        ))
    }

    fn tokenize_batch(&self, samples: &[RawSample]) -> TokenizerResult<TokenizedBatch> {
        samples
            .iter()
            .map(|sample| self.tokenize_sample(sample))
            .collect::<TokenizerResult<Vec<_>>>()
            .map(|samples| TokenizedBatch::new(0, samples))
    }
}

fn raw_batch() -> RawSampleBatch {
    RawSampleBatch::new(
        0,
        (0..SAMPLE_COUNT)
            .map(|index| RawSample::with_id(index.to_string(), Bytes::from_static(b"hello world")))
            .collect(),
        false,
    )
}

fn formatted_batch() -> RawSampleBatch {
    let formatter = TemplateFormatter::new(
        ChatTemplate::new(ChatTemplateConfig::new(TemplateKind::Llama3)),
        TextSpecialTokens::default(),
    );
    let messages = vec![ChatMessage::user("hello world")];
    let formatted = formatter.format(&messages).expect("formatted");
    RawSampleBatch::new(
        0,
        (0..SAMPLE_COUNT)
            .map(|index| {
                RawSample::with_id(
                    index.to_string(),
                    Bytes::copy_from_slice(formatted.as_bytes()),
                )
            })
            .collect(),
        false,
    )
}

fn bench_end_to_end(c: &mut Criterion) {
    let raw = raw_batch();
    let formatted = formatted_batch();
    let config = TokenizerConfig {
        tokenizer_name: Some("bench-tokenizer".to_owned()),
        max_seq_len: Some(4096),
        ..TokenizerConfig::default()
    };
    let mut group = c.benchmark_group("end_to_end");
    group.throughput(Throughput::Elements(SAMPLE_COUNT as u64));

    group.bench_function("raw_to_tokenized_batch", |b| {
        b.iter(|| {
            let mut adapter = TokenizerStreamAdapter::new(EndToEndTokenizer, SAMPLE_COUNT);
            let output = adapter.tokenize_batch(raw.clone()).expect("stream");
            black_box(output.batch.metadata.tokens);
        });
    });

    group.bench_function("template_to_tokenized_batch", |b| {
        b.iter(|| {
            let mut adapter = TokenizerStreamAdapter::new(EndToEndTokenizer, SAMPLE_COUNT);
            let output = adapter.tokenize_batch(formatted.clone()).expect("stream");
            black_box(output.batch.metadata.tokens);
        });
    });

    group.bench_function("cache_then_tokenize", |b| {
        b.iter(|| {
            let mut cache = MemoryTokenizerCache::new(SAMPLE_COUNT);
            for sample in &raw.samples {
                let key = TokenizerCacheKey::from_text(
                    &sample.payload,
                    "bench-fingerprint",
                    &config,
                    CacheTarget::RawText,
                );
                if cache.get(&key).is_none() {
                    cache.insert(key, TokenizerCacheEntry::TokenIds(vec![1, 2, 3]));
                }
            }
            black_box(cache.stats().hit_ratio());
        });
    });

    group.finish();
}

criterion_group!(benches, bench_end_to_end);
criterion_main!(benches);
