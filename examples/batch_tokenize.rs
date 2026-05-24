use bytes::Bytes;
use tunex_core::types::{RawSample, TokenizedSample};
use tunex_tokenizer::prelude::*;

#[derive(Clone, Copy)]
struct ExampleTokenizer;

impl TunexTokenizer for ExampleTokenizer {
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

fn main() -> TokenizerResult<()> {
    let samples = vec![
        RawSample::with_id("a", Bytes::from_static(b"hello")),
        RawSample::with_id("b", Bytes::from_static(b"world")),
    ];
    let batch = ExampleTokenizer.tokenize_batch(&samples)?;

    println!(
        "samples={} tokens={}",
        batch.metadata.samples, batch.metadata.tokens
    );
    Ok(())
}
