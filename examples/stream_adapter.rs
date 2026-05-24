use bytes::Bytes;
use tunex_core::types::{RawSample, TokenizedSample};
use tunex_tokenizer::prelude::*;

#[derive(Clone, Copy)]
struct ExampleTokenizer;

impl TunexTokenizer for ExampleTokenizer {
    fn tokenize_sample(&self, sample: &RawSample) -> TokenizerResult<TokenizedSample> {
        Ok(TokenizedSample::new(
            sample.payload.iter().map(|byte| u32::from(*byte)).collect(),
        ))
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
    let input = RawSampleBatch::new(
        7,
        vec![RawSample::with_id("a", Bytes::from_static(b"stream me"))],
        true,
    );
    let mut adapter =
        TokenizerStreamAdapter::new(ExampleTokenizer, 64).with_queue_capacity(QueueCapacity {
            capacity: 8,
            len: 0,
        });
    let output = adapter.tokenize_batch(input)?;

    println!(
        "input_batch={} output_batch={} tokens={}",
        output.input_batch_id, output.output_batch_id, output.batch.metadata.tokens
    );
    Ok(())
}
