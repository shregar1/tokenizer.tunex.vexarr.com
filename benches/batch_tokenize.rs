use std::{
    alloc::{GlobalAlloc, Layout, System},
    hint::black_box,
    sync::atomic::{AtomicUsize, Ordering},
};

use bytes::Bytes;
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use tunex_core::types::{RawSample, TokenizedSample};
use tunex_tokenizer::{
    BatchTokenizationScheduler, ParallelBatchTokenizer, TokenizedBatch, TokenizerResult,
    TunexTokenizer,
};

const SAMPLE_COUNT: usize = 32_768;

static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);

struct CountingAllocator;

// SAFETY: This allocator delegates every allocation operation to the standard
// system allocator and only increments an atomic counter before allocation.
unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        // SAFETY: The layout is forwarded unchanged to the system allocator.
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: The pointer and layout are forwarded unchanged to the system
        // allocator that originally allocated the memory.
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static GLOBAL: CountingAllocator = CountingAllocator;

#[derive(Clone, Copy)]
struct BenchTokenizer;

impl TunexTokenizer for BenchTokenizer {
    fn tokenize_sample(&self, sample: &RawSample) -> TokenizerResult<TokenizedSample> {
        let mut output = TokenizedSample::new(
            sample
                .payload
                .iter()
                .map(|byte| u32::from(*byte))
                .collect::<Vec<_>>(),
        );
        output.id = sample.id.clone();
        Ok(output)
    }

    fn tokenize_batch(&self, samples: &[RawSample]) -> TokenizerResult<TokenizedBatch> {
        let outputs = samples
            .iter()
            .map(|sample| self.tokenize_sample(sample))
            .collect::<TokenizerResult<Vec<_>>>()?;
        Ok(TokenizedBatch::new(0, outputs))
    }
}

fn samples() -> Vec<RawSample> {
    (0..SAMPLE_COUNT)
        .map(|index| RawSample::with_id(index.to_string(), Bytes::from_static(b"hello world")))
        .collect()
}

fn bench_batch_tokenize(c: &mut Criterion) {
    let input = samples();
    let tokenizer = BenchTokenizer;
    let bytes = u64::try_from(SAMPLE_COUNT * b"hello world".len()).expect("bytes");
    let mut group = c.benchmark_group("batch_tokenize");
    group.throughput(Throughput::Bytes(bytes));

    for workers in [1, 2, 4, 8] {
        group.bench_function(format!("parallel_workers_{workers}"), |b| {
            b.iter(|| {
                let scheduler = BatchTokenizationScheduler::new(2048, workers).expect("scheduler");
                let parallel = ParallelBatchTokenizer::new(&tokenizer, scheduler);
                let (batch, stats) = parallel.tokenize_ordered(&input).expect("batch");
                black_box((
                    batch.metadata.samples,
                    stats.tokens_per_second(),
                    stats.samples_per_second(),
                    stats.latency,
                ));
            });
        });
    }

    group.bench_function("allocations_per_sample", |b| {
        b.iter(|| {
            ALLOCATIONS.store(0, Ordering::Relaxed);
            let scheduler = BatchTokenizationScheduler::new(2048, 4).expect("scheduler");
            let parallel = ParallelBatchTokenizer::new(&tokenizer, scheduler);
            let (batch, _) = parallel.tokenize_ordered(&input).expect("batch");
            black_box((
                batch.metadata.tokens,
                ALLOCATIONS.load(Ordering::Relaxed) as f64 / SAMPLE_COUNT as f64,
            ));
        });
    });

    group.finish();
}

criterion_group!(benches, bench_batch_tokenize);
criterion_main!(benches);
