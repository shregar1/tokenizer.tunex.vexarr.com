use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use tunex_tokenizer::{
    CacheTarget, MemoryTokenizerCache, TokenizerCacheEntry, TokenizerCacheKey, TokenizerConfig,
};

const LOOKUPS: usize = 16_384;

fn keys(config: &TokenizerConfig) -> Vec<TokenizerCacheKey> {
    (0..LOOKUPS)
        .map(|index| {
            TokenizerCacheKey::from_text(
                format!("sample-{index}"),
                "bench-fingerprint",
                config,
                CacheTarget::RawText,
            )
        })
        .collect()
}

fn bench_cache_lookup(c: &mut Criterion) {
    let config = TokenizerConfig {
        tokenizer_name: Some("bench-tokenizer".to_owned()),
        max_seq_len: Some(4096),
        ..TokenizerConfig::default()
    };
    let keys = keys(&config);
    let mut group = c.benchmark_group("cache_lookup");
    group.throughput(Throughput::Elements(LOOKUPS as u64));

    group.bench_function("cache_hit_latency", |b| {
        b.iter(|| {
            let mut cache = MemoryTokenizerCache::new(LOOKUPS);
            for key in &keys {
                cache.insert(key.clone(), TokenizerCacheEntry::TokenIds(vec![1, 2, 3]));
            }
            for key in &keys {
                black_box(cache.get(key));
            }
            black_box(cache.stats().hit_ratio());
        });
    });

    group.bench_function("no_cache_baseline", |b| {
        b.iter(|| {
            let mut tokens = 0usize;
            for _ in &keys {
                tokens += [1_u32, 2, 3].len();
            }
            black_box(tokens);
        });
    });

    group.finish();
}

criterion_group!(benches, bench_cache_lookup);
criterion_main!(benches);
