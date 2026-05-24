use std::hint::black_box;

use bytes::Bytes;
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use tunex_core::types::{RawSample, TokenizedSample};
use tunex_tokenizer::{TokenizedBatch, TokenizerResult, TunexTokenizer};

const SAMPLE_COUNT: usize = 16_384;

#[derive(Clone, Copy)]
struct BenchTokenizer;

impl TunexTokenizer for BenchTokenizer {
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

fn samples() -> Vec<RawSample> {
    (0..SAMPLE_COUNT)
        .map(|index| RawSample::with_id(index.to_string(), Bytes::from_static(b"hello world")))
        .collect()
}

fn bench_hf_tokenize(c: &mut Criterion) {
    let input = samples();
    let tokenizer = BenchTokenizer;
    let mut group = c.benchmark_group("hf_tokenize");
    group.throughput(Throughput::Elements(SAMPLE_COUNT as u64));
    group.bench_function("mock_hf_batch_contract", |b| {
        b.iter(|| {
            let batch = tokenizer.tokenize_batch(&input).expect("batch");
            black_box((batch.metadata.samples, batch.metadata.tokens));
        });
    });
    group.finish();
}

criterion_group!(benches, bench_hf_tokenize);
criterion_main!(benches);
