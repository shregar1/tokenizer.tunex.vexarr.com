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
    let epoch = DistributedTokenizerEpoch::new(0, 42);
    let rank = DistributedTokenizerRank::new(0, 1)?;
    let shard = TokenizerShard::new(rank, epoch, 0, 0, 10);
    let assignment = TokenizerWorkerAssignment::new(TokenizerWorkerId(0), rank, epoch, vec![shard]);
    let adapter = TokenizerStreamAdapter::new(ExampleTokenizer, 2);
    let mut worker = DistributedTokenizerWorker::new(assignment, adapter);
    let batch = RawSampleBatch::new(
        0,
        vec![RawSample::with_id("a", Bytes::from_static(b"distributed"))],
        false,
    );
    let output = worker.tokenize_owned_batch(batch)?;

    println!("distributed_batch_id={}", output.distributed_batch_id);
    Ok(())
}
