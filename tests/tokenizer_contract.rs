use tunex_core::{plugin::TunexPlugin, types::TokenizedSample};
use tunex_tokenizer::{TokenizedBatch, TokenizerPlugin, TOKENIZER_PLUGIN_NAME};

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
