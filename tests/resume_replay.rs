use bytes::Bytes;
use tunex_core::types::{RawSample, TokenizedSample};
use tunex_tokenizer::{
    DistributedTokenizerRank, ReplayPlan, ReplayVerifier, TokenizedBatch, TokenizerCursor,
    TokenizerResult, TokenizerResumeSnapshot, TokenizerResumeState, TunexTokenizer,
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
fn cursor_tracks_resume_position_and_rank() {
    let rank = DistributedTokenizerRank::new(1, 4).expect("rank");
    let cursor = TokenizerCursor::new(2, 99, "tok-v1", "template-v1")
        .with_rank(rank, 7)
        .advance_batch(128);

    assert_eq!(cursor.epoch, 2);
    assert_eq!(cursor.sample_index, 128);
    assert_eq!(cursor.batch_index, 1);
    assert_eq!(cursor.rank, Some(1));
    assert_eq!(cursor.shard_id, Some(7));
    assert_eq!(cursor.distributed_epoch().seed, 99);
}

#[test]
fn snapshot_validates_distributed_rank_alignment() {
    let cursor = TokenizerCursor::new(0, 42, "tok", "template")
        .with_rank(DistributedTokenizerRank::new(2, 4).expect("rank"), 2);
    let state = TokenizerResumeState::new(cursor, "config").with_distributed(4, Some(123));
    let snapshot = TokenizerResumeSnapshot::new(state).expect("snapshot");

    assert_eq!(snapshot.version, TokenizerResumeSnapshot::VERSION);
    assert_eq!(snapshot.cursor().rank, Some(2));
}

#[test]
fn invalid_resume_state_reports_missing_rank_for_world_size() {
    let cursor = TokenizerCursor::new(0, 42, "tok", "template");
    let state = TokenizerResumeState::new(cursor, "config").with_distributed(2, None);
    let report = state.validation_report();

    assert!(!report.is_valid());
    assert!(report
        .errors
        .iter()
        .any(|error| error.path == "cursor.rank"));
}

#[test]
fn replay_verifier_confirms_same_input_same_tokens() {
    let tokenizer = StableTokenizer;
    let samples = vec![sample("a", b"same"), sample("b", b"tokens")];
    let expected = tokenizer.tokenize_batch(&samples).expect("expected");
    let cursor = TokenizerCursor::new(0, 42, "tok", "template").advance_batch(2);
    let state = TokenizerResumeState::new(cursor, "config").with_cache_fingerprint("cache");
    let plan = ReplayPlan::new(state).expect("plan");
    let verifier = ReplayVerifier::new(plan);

    let replay = verifier
        .verify_batch(&tokenizer, &samples, &expected, "tok", "template")
        .expect("replay");

    assert!(replay.matched);
    assert_eq!(replay.batch.input_ids, expected.input_ids);
    assert_eq!(replay.batch.sample_ids, expected.sample_ids);
}

#[test]
fn replay_verifier_rejects_fingerprint_mismatch() {
    let tokenizer = StableTokenizer;
    let samples = vec![sample("a", b"same")];
    let expected = tokenizer.tokenize_batch(&samples).expect("expected");
    let cursor = TokenizerCursor::new(0, 42, "tok", "template");
    let state = TokenizerResumeState::new(cursor, "config");
    let plan = ReplayPlan::new(state).expect("plan");
    let verifier = ReplayVerifier::new(plan);

    let error = verifier
        .verify_batch(&tokenizer, &samples, &expected, "other", "template")
        .expect_err("fingerprint mismatch");

    assert!(error.to_string().contains("fingerprint mismatch"));
}
