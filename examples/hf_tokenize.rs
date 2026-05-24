use bytes::Bytes;
use tunex_core::types::RawSample;
use tunex_tokenizer::prelude::*;

fn main() -> TokenizerResult<()> {
    let config = TokenizerConfig {
        tokenizer_path: Some("tokenizer.json".into()),
        max_seq_len: Some(4096),
        truncate: true,
        ..TokenizerConfig::default()
    };
    let tokenizer = HfTokenizer::from_config(config)?;
    let samples = vec![RawSample::with_id(
        "sample-1",
        Bytes::from_static(b"hello world"),
    )];
    let batch = tokenizer.tokenize_batch(&samples)?;

    println!(
        "samples={} tokens={}",
        batch.metadata.samples, batch.metadata.tokens
    );
    Ok(())
}
