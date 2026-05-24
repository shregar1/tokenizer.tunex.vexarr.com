use tunex_core::types::TokenizedSample;
use tunex_tokenizer::{
    CachePolicy, CacheTarget, MemoryTokenizerCache, TokenizedBatch, TokenizerCacheEntry,
    TokenizerCacheKey, TokenizerConfig,
};

#[test]
fn cache_key_includes_tokenizer_settings() {
    let base = TokenizerConfig {
        tokenizer_name: Some("model-a".to_owned()),
        max_seq_len: Some(128),
        truncate: true,
        add_bos: true,
        bos_token_id: Some(1),
        ..TokenizerConfig::default()
    };
    let changed = TokenizerConfig {
        max_seq_len: Some(256),
        ..base.clone()
    };

    let first = TokenizerCacheKey::from_text("hello", "fingerprint", &base, CacheTarget::RawText);
    let second =
        TokenizerCacheKey::from_text("hello", "fingerprint", &changed, CacheTarget::RawText);

    assert_ne!(first, second);
}

#[test]
fn memory_cache_hits_misses_and_eviction_are_tracked() {
    let config = TokenizerConfig {
        tokenizer_name: Some("model-a".to_owned()),
        ..TokenizerConfig::default()
    };
    let first = TokenizerCacheKey::from_text("one", "fingerprint", &config, CacheTarget::RawText);
    let second = TokenizerCacheKey::from_text("two", "fingerprint", &config, CacheTarget::RawText);
    let mut cache = MemoryTokenizerCache::new(1);

    assert!(cache.get(&first).is_none());
    cache.insert(first.clone(), TokenizerCacheEntry::TokenIds(vec![1, 2, 3]));
    assert_eq!(
        cache.get(&first),
        Some(TokenizerCacheEntry::TokenIds(vec![1, 2, 3]))
    );

    cache.insert(second, TokenizerCacheEntry::TokenIds(vec![4]));

    assert!(cache.get(&first).is_none());
    assert_eq!(cache.len(), 1);
    assert_eq!(cache.stats().hits, 1);
    assert_eq!(cache.stats().misses, 2);
    assert_eq!(cache.stats().evictions, 1);
}

#[test]
fn cache_policy_defaults_to_enabled_memory_lru() {
    let policy = CachePolicy::default();

    assert!(policy.is_enabled());
}

#[test]
fn cache_entry_estimates_batch_bytes() {
    let batch = TokenizedBatch::new(1, vec![TokenizedSample::new(vec![1, 2, 3])]);
    let entry = TokenizerCacheEntry::TokenizedBatch(batch);

    assert!(entry.estimated_bytes() > 0);
}
