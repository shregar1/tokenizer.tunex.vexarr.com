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
    let samples = (0..1024)
        .map(|index| RawSample::with_id(index.to_string(), Bytes::from_static(b"hello world")))
        .collect::<Vec<_>>();
    let scheduler = BatchTokenizationScheduler::new(128, 4)?;
    let parallel = ParallelBatchTokenizer::new(&ExampleTokenizer, scheduler);
    let (batch, stats) = parallel.tokenize_ordered(&samples)?;

    println!(
        "samples={} tokens/sec={:.2}",
        batch.metadata.samples,
        stats.tokens_per_second()
    );
    Ok(())
}
