use std::hint::black_box;

use bytes::Bytes;
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use tunex_core::types::{RawSample, TokenizedSample};
use tunex_tokenizer::{TokenizedBatch, TokenizerResult, TunexTokenizer};

const SAMPLE_COUNT: usize = 32_768;

#[derive(Clone, Copy)]
struct BatchTokenizer;

impl TunexTokenizer for BatchTokenizer {
    fn tokenize_sample(&self, sample: &RawSample) -> TokenizerResult<TokenizedSample> {
        let mut output =
            TokenizedSample::new(sample.payload.iter().map(|byte| u32::from(*byte)).collect());
        output.id = sample.id.clone();
        Ok(output)
    }

    fn tokenize_batch(&self, samples: &[RawSample]) -> TokenizerResult<TokenizedBatch> {
        samples
            .iter()
            .map(|sample| self.tokenize_sample(sample))
            .collect::<TokenizerResult<Vec<_>>>()
            .map(|samples| TokenizedBatch::new(0, samples))
    }
}

fn samples() -> Vec<RawSample> {
    (0..SAMPLE_COUNT)
        .map(|index| RawSample::with_id(index.to_string(), Bytes::from_static(b"hello world")))
        .collect()
}

fn bench_hf_batch(c: &mut Criterion) {
    let input = samples();
    let tokenizer = BatchTokenizer;
    let mut group = c.benchmark_group("hf_batch");
    group.throughput(Throughput::Elements(SAMPLE_COUNT as u64));

    group.bench_function("batch_tokens_per_sec", |b| {
        b.iter(|| {
            let batch = tokenizer.tokenize_batch(&input).expect("batch");
            black_box((batch.metadata.samples, batch.metadata.tokens));
        });
    });

    group.finish();
}

criterion_group!(benches, bench_hf_batch);
criterion_main!(benches);
