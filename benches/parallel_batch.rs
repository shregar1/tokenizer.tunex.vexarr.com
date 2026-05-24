use std::hint::black_box;

use bytes::Bytes;
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use tunex_core::types::{RawSample, TokenizedSample};
use tunex_tokenizer::{
    BatchTokenizationScheduler, ParallelBatchTokenizer, TokenizedBatch, TokenizerResult,
    TunexTokenizer,
};

const SAMPLE_COUNT: usize = 32_768;

#[derive(Clone, Copy)]
struct ParallelBenchTokenizer;

impl TunexTokenizer for ParallelBenchTokenizer {
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

fn bench_parallel_batch(c: &mut Criterion) {
    let input = samples();
    let tokenizer = ParallelBenchTokenizer;
    let mut group = c.benchmark_group("parallel_batch");
    group.throughput(Throughput::Elements(SAMPLE_COUNT as u64));

    group.bench_function("sequential_baseline", |b| {
        b.iter(|| {
            let batch = tokenizer.tokenize_batch(&input).expect("batch");
            black_box((batch.metadata.tokens, batch.metadata.samples));
        });
    });

    for workers in [1, 2, 4, 8] {
        group.bench_function(format!("parallel_workers_{workers}"), |b| {
            b.iter(|| {
                let scheduler = BatchTokenizationScheduler::new(2048, workers).expect("scheduler");
                let parallel = ParallelBatchTokenizer::new(&tokenizer, scheduler);
                let (batch, stats) = parallel.tokenize_ordered(&input).expect("parallel batch");
                black_box((
                    batch.metadata.tokens,
                    stats.samples_per_second(),
                    stats.tokens_per_second(),
                ));
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_parallel_batch);
criterion_main!(benches);
