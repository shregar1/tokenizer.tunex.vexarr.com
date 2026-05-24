use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use tunex_tokenizer::{
    CacheTarget, MemoryTokenizerCache, TokenizerCacheEntry, TokenizerCacheKey, TokenizerConfig,
};

const LOOKUPS: usize = 16_384;

fn bench_cache(c: &mut Criterion) {
    let config = TokenizerConfig {
        tokenizer_name: Some("bench-tokenizer".to_owned()),
        max_seq_len: Some(4096),
        ..TokenizerConfig::default()
    };
    let keys = (0..LOOKUPS)
        .map(|index| {
            TokenizerCacheKey::from_text(
                format!("sample-{index}"),
                "bench-fingerprint",
                &config,
                CacheTarget::RawText,
            )
        })
        .collect::<Vec<_>>();
    let mut group = c.benchmark_group("cache");
    group.throughput(Throughput::Elements(LOOKUPS as u64));
    group.bench_function("memory_lru_lookup", |b| {
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
    group.finish();
}

criterion_group!(benches, bench_cache);
criterion_main!(benches);
