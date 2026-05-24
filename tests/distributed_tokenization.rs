use bytes::Bytes;
use tunex_core::types::{RawSample, TokenizedSample};
use tunex_tokenizer::{
    DistributedTokenBatch, DistributedTokenizerEpoch, DistributedTokenizerRank,
    DistributedTokenizerWorker, RawSampleBatch, RoutingStrategy, TokenizedBatch, TokenizerResult,
    TokenizerShard, TokenizerStreamAdapter, TokenizerWorkerAssignment, TokenizerWorkerId,
    TunexTokenizer,
};

#[derive(Clone, Copy)]
struct StableTokenizer;

impl TunexTokenizer for StableTokenizer {
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

fn sample(id: &str, text: &'static [u8]) -> RawSample {
    RawSample::with_id(id, Bytes::from_static(text))
}

#[test]
fn routing_by_shard_ownership_is_deterministic() {
    let epoch = DistributedTokenizerEpoch::new(3, 99);
    let rank0 = DistributedTokenizerRank::new(0, 3).expect("rank");
    let rank1 = DistributedTokenizerRank::new(1, 3).expect("rank");
    let shards = vec![
        TokenizerShard::new(rank0, epoch, 0, 0, 10),
        TokenizerShard::new(rank1, epoch, 1, 10, 10),
    ];
    let assignment = TokenizerWorkerAssignment::new(TokenizerWorkerId(0), rank0, epoch, shards);
    let router = assignment.router(RoutingStrategy::ShardOwnership);
    let batch = RawSampleBatch::new(12, vec![sample("a", b"abc")], false);

    let left = router.route(&batch).expect("route");
    let right = router.route(&batch).expect("route");

    assert_eq!(left, right);
    assert_eq!(left.rank, rank1);
    assert_eq!(left.shard_id, Some(1));
}

#[test]
fn distributed_worker_derives_resume_safe_batch_id() {
    let epoch = DistributedTokenizerEpoch::new(1, 42);
    let rank = DistributedTokenizerRank::new(0, 1).expect("rank");
    let shard = TokenizerShard::new(rank, epoch, 0, 0, 100);
    let assignment = TokenizerWorkerAssignment::new(TokenizerWorkerId(7), rank, epoch, vec![shard]);
    let adapter = TokenizerStreamAdapter::new(StableTokenizer, 2);
    let mut worker = DistributedTokenizerWorker::new(assignment, adapter);
    let batch = RawSampleBatch::new(5, vec![sample("a", b"abc"), sample("b", b"de")], false);

    let output = worker.tokenize_owned_batch(batch).expect("distributed");

    assert_eq!(output.route.rank, rank);
    assert_eq!(output.envelope.input_batch_id, 5);
    assert_eq!(output.envelope.batch.input_ids, vec![97, 98, 99, 100, 101]);
    assert_eq!(
        output.distributed_batch_id,
        DistributedTokenBatch::new(output.route.clone(), epoch, output.envelope.clone())
            .distributed_batch_id
    );
}

#[test]
fn same_input_shard_produces_same_tokens() {
    let epoch = DistributedTokenizerEpoch::default();
    let rank = DistributedTokenizerRank::new(0, 1).expect("rank");
    let shard = TokenizerShard::new(rank, epoch, 0, 0, 10);
    let assignment = TokenizerWorkerAssignment::new(TokenizerWorkerId(1), rank, epoch, vec![shard]);
    let batch = RawSampleBatch::new(2, vec![sample("same", b"same")], false);

    let mut left = DistributedTokenizerWorker::new(
        assignment.clone(),
        TokenizerStreamAdapter::new(StableTokenizer, 1),
    );
    let mut right = DistributedTokenizerWorker::new(
        assignment,
        TokenizerStreamAdapter::new(StableTokenizer, 1),
    );

    let left_output = left.tokenize_owned_batch(batch.clone()).expect("left");
    let right_output = right.tokenize_owned_batch(batch).expect("right");

    assert_eq!(
        left_output.envelope.batch.input_ids,
        right_output.envelope.batch.input_ids
    );
    assert_eq!(
        left_output.distributed_batch_id,
        right_output.distributed_batch_id
    );
}
