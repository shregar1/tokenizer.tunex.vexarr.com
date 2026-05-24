use tokenizers::{Encoding, Token};
use tunex_core::{plugin::TunexPlugin, types::TokenizedSample};
use tunex_tokenizer::{
    EncodingAdapter, SpecialTokenConfig, TokenizedBatch, TokenizerPlugin, TOKENIZER_PLUGIN_NAME,
};

#[test]
fn tokenized_batch_records_sample_and_token_counts() {
    let batch = TokenizedBatch::new(7, vec![TokenizedSample::new(vec![1, 2, 3])]);

    assert_eq!(batch.len(), 1);
    assert_eq!(batch.metadata.batch_id, 7);
    assert_eq!(batch.metadata.samples, 1);
    assert_eq!(batch.metadata.tokens, 3);
}

#[test]
fn tokenizer_plugin_exposes_core_metadata() {
    let plugin = TokenizerPlugin::new();
    let metadata = plugin.metadata();

    assert_eq!(metadata.id.as_str(), TOKENIZER_PLUGIN_NAME);
    assert!(plugin
        .capabilities()
        .iter()
        .any(|capability| capability.as_str() == "hf-tokenizer"));
}

#[test]
fn encoding_adapter_applies_bos_eos_and_truncation() {
    let encoding = Encoding::from_tokens(
        vec![
            Token::new(10, "hello".to_owned(), (0, 5)),
            Token::new(11, "world".to_owned(), (6, 11)),
        ],
        0,
    );
    let adapter = EncodingAdapter::new(SpecialTokenConfig::new(Some(1), Some(2)), Some(3), true);

    let sample = adapter.convert(Some("sample-1".to_owned()), &encoding);

    assert_eq!(sample.id.as_deref(), Some("sample-1"));
    assert_eq!(sample.token_ids, vec![1, 10, 11]);
    assert_eq!(sample.attention_mask, vec![1, 1, 1]);
}

#[test]
fn encoding_adapter_keeps_full_sequence_without_truncation() {
    let encoding = Encoding::from_tokens(
        vec![
            Token::new(10, "hello".to_owned(), (0, 5)),
            Token::new(11, "world".to_owned(), (6, 11)),
        ],
        0,
    );
    let adapter = EncodingAdapter::new(SpecialTokenConfig::new(Some(1), Some(2)), Some(3), false);

    let sample = adapter.convert(None, &encoding);

    assert_eq!(sample.token_ids, vec![1, 10, 11, 2]);
    assert_eq!(sample.attention_mask, vec![1, 1, 1, 1]);
}
