use tunex_tokenizer::prelude::*;

fn main() {
    let config = TokenizerConfig {
        tokenizer_name: Some("example-tokenizer".to_owned()),
        max_seq_len: Some(4096),
        ..TokenizerConfig::default()
    };
    let key = TokenizerCacheKey::from_text(
        "hello world",
        "tokenizer-fingerprint",
        &config,
        CacheTarget::RawText,
    );
    let mut cache = MemoryTokenizerCache::new(1024);

    cache.insert(key.clone(), TokenizerCacheEntry::TokenIds(vec![1, 2, 3]));
    let cached = cache.get(&key);

    println!(
        "hit={} hit_ratio={:.2}",
        cached.is_some(),
        cache.stats().hit_ratio()
    );
}
