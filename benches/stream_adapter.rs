use std::hint::black_box;

use bytes::Bytes;
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use tunex_core::types::{RawSample, TokenizedSample};
use tunex_tokenizer::{
    QueueCapacity, RawSampleBatch, TokenizedBatch, TokenizerResult, TokenizerStreamAdapter,
    TunexTokenizer,
};

const SAMPLE_COUNT: usize = 4096;

#[derive(Clone, Copy)]
struct StreamBenchTokenizer;

impl TunexTokenizer for StreamBenchTokenizer {
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

fn batch() -> RawSampleBatch {
    RawSampleBatch::new(
        0,
        (0..SAMPLE_COUNT)
            .map(|index| RawSample::with_id(index.to_string(), Bytes::from_static(b"hello world")))
            .collect(),
        false,
    )
}

fn bench_stream_adapter(c: &mut Criterion) {
    let input = batch();
    let mut group = c.benchmark_group("stream_adapter");
    group.throughput(Throughput::Elements(SAMPLE_COUNT as u64));

    group.bench_function("sync_adapter", |b| {
        b.iter(|| {
            let mut adapter = TokenizerStreamAdapter::new(StreamBenchTokenizer, SAMPLE_COUNT);
            let output = adapter.tokenize_batch(input.clone()).expect("stream");
            black_box((output.output_batch_id, output.batch.metadata.tokens));
        });
    });

    group.bench_function("bounded_queue_metadata", |b| {
        b.iter(|| {
            let mut adapter = TokenizerStreamAdapter::new(StreamBenchTokenizer, SAMPLE_COUNT)
                .with_queue_capacity(QueueCapacity {
                    capacity: 16,
                    len: 8,
                });
            let output = adapter.tokenize_batch(input.clone()).expect("stream");
            black_box((output.stats.input_batches, output.stats.output_tokens));
        });
    });

    group.finish();
}

criterion_group!(benches, bench_stream_adapter);
criterion_main!(benches);
