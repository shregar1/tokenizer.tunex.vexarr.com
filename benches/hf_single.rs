use std::{
    alloc::{GlobalAlloc, Layout, System},
    hint::black_box,
    sync::atomic::{AtomicUsize, Ordering},
};

use bytes::Bytes;
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use tunex_core::types::{RawSample, TokenizedSample};
use tunex_tokenizer::{TokenizedBatch, TokenizerResult, TunexTokenizer};

const SAMPLE_COUNT: usize = 16_384;

static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);

struct CountingAllocator;

// SAFETY: This allocator forwards all allocation operations unchanged to the
// system allocator and only records an atomic allocation count.
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
struct SingleTokenizer;

impl TunexTokenizer for SingleTokenizer {
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

fn bench_hf_single(c: &mut Criterion) {
    let input = samples();
    let tokenizer = SingleTokenizer;
    let mut group = c.benchmark_group("hf_single");
    group.throughput(Throughput::Elements(SAMPLE_COUNT as u64));

    group.bench_function("single_thread_samples_per_sec", |b| {
        b.iter(|| {
            let mut tokens = 0usize;
            for sample in &input {
                tokens += tokenizer
                    .tokenize_sample(sample)
                    .expect("tokenized")
                    .token_ids
                    .len();
            }
            black_box(tokens);
        });
    });

    group.bench_function("allocations_per_sample", |b| {
        b.iter(|| {
            ALLOCATIONS.store(0, Ordering::Relaxed);
            let mut tokens = 0usize;
            for sample in &input {
                tokens += tokenizer
                    .tokenize_sample(sample)
                    .expect("tokenized")
                    .token_ids
                    .len();
            }
            black_box((
                tokens,
                ALLOCATIONS.load(Ordering::Relaxed) as f64 / SAMPLE_COUNT as f64,
            ));
        });
    });

    group.finish();
}

criterion_group!(benches, bench_hf_single);
criterion_main!(benches);
