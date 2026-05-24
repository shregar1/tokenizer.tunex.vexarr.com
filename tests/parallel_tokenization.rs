use bytes::Bytes;
use tunex_core::types::{RawSample, TokenizedSample};
use tunex_tokenizer::{
    BatchTokenizationScheduler, ParallelBatchTokenizer, TokenizedBatch, TokenizerResult,
    TunexTokenizer,
};

#[derive(Clone, Copy)]
struct FakeTokenizer;

impl TunexTokenizer for FakeTokenizer {
    fn tokenize_sample(&self, sample: &RawSample) -> TokenizerResult<TokenizedSample> {
        let id = sample.id.clone();
        let token_ids = sample.payload.iter().map(|byte| u32::from(*byte)).collect();
        let mut output = TokenizedSample::new(token_ids);
        output.id = id;
        Ok(output)
    }

    fn tokenize_batch(&self, samples: &[RawSample]) -> TokenizerResult<TokenizedBatch> {
        let outputs = samples
            .iter()
            .map(|sample| self.tokenize_sample(sample))
            .collect::<TokenizerResult<Vec<_>>>()?;
        Ok(TokenizedBatch::new(0, outputs))
    }
}

#[test]
fn scheduler_builds_bounded_contiguous_chunks() {
    let scheduler = BatchTokenizationScheduler::new(3, 2).expect("scheduler");
    let plan = scheduler.plan(8);

    assert_eq!(plan.chunks.len(), 3);
    assert_eq!(plan.chunks[0].start, 0);
    assert_eq!(plan.chunks[0].end, 3);
    assert_eq!(plan.chunks[2].start, 6);
    assert_eq!(plan.chunks[2].end, 8);
}

#[test]
fn parallel_tokenization_preserves_input_order() {
    let samples = (0..16)
        .map(|index| RawSample::with_id(index.to_string(), Bytes::from(index.to_string())))
        .collect::<Vec<_>>();
    let scheduler = BatchTokenizationScheduler::new(4, 4).expect("scheduler");
    let tokenizer = FakeTokenizer;
    let parallel = ParallelBatchTokenizer::new(&tokenizer, scheduler);

    let (batch, stats) = parallel.tokenize_ordered(&samples).expect("batch");

    assert_eq!(batch.len(), samples.len());
    assert_eq!(stats.workers, 4);
    assert_eq!(stats.chunks, 4);
    let output_samples = batch.to_samples();
    for (index, sample) in output_samples.iter().enumerate() {
        assert_eq!(sample.id.as_deref(), Some(index.to_string()).as_deref());
    }
}

#[test]
fn same_config_produces_same_tokens() {
    let samples = (0..8)
        .map(|index| RawSample::with_id(index.to_string(), Bytes::from_static(b"abc")))
        .collect::<Vec<_>>();
    let tokenizer = FakeTokenizer;
    let left = ParallelBatchTokenizer::new(
        &tokenizer,
        BatchTokenizationScheduler::new(2, 2).expect("scheduler"),
    )
    .tokenize_ordered(&samples)
    .expect("left")
    .0;
    let right = ParallelBatchTokenizer::new(
        &tokenizer,
        BatchTokenizationScheduler::new(2, 2).expect("scheduler"),
    )
    .tokenize_ordered(&samples)
    .expect("right")
    .0;

    assert_eq!(left.to_samples(), right.to_samples());
}
