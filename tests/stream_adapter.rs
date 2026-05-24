use bytes::Bytes;
use tunex_core::types::{RawSample, TokenizedSample};
use tunex_tokenizer::{
    BackpressureAction, QueueCapacity, RawSampleBatch, TokenizedBatch, TokenizerError,
    TokenizerResult, TokenizerStreamAdapter, TunexTokenizer,
};

#[derive(Clone, Copy)]
struct LengthTokenizer;

impl TunexTokenizer for LengthTokenizer {
    fn tokenize_sample(&self, sample: &RawSample) -> TokenizerResult<TokenizedSample> {
        if sample.payload == Bytes::from_static(b"bad") {
            return Err(TokenizerError::InvalidUtf8);
        }

        let len = u32::try_from(sample.payload.len()).expect("test payload length fits");
        let mut output = TokenizedSample::new(vec![len]);
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
fn stream_adapter_preserves_sample_order_and_batch_ids() {
    let mut adapter = TokenizerStreamAdapter::new(LengthTokenizer, 4);
    let input = RawSampleBatch::new(42, vec![sample("a", b"one"), sample("b", b"three")], true);

    let output = adapter.tokenize_batch(input).expect("tokenized");

    assert_eq!(output.input_batch_id, 42);
    assert_eq!(output.output_batch_id, 0);
    assert_eq!(
        output.batch.sample_ids,
        vec![Some("a".to_owned()), Some("b".to_owned())]
    );
    assert_eq!(output.batch.input_ids, vec![3, 5]);
    assert_eq!(output.stats.input_batches, 1);
    assert_eq!(output.stats.output_batches, 1);
    assert_eq!(output.stats.partial_batches, 1);
}

#[test]
fn stream_adapter_reports_sample_index_on_error() {
    let mut adapter = TokenizerStreamAdapter::new(LengthTokenizer, 4);
    let input = RawSampleBatch::new(7, vec![sample("ok", b"ok"), sample("bad", b"bad")], false);

    let error = adapter.tokenize_batch(input).expect_err("sample error");

    match error {
        TokenizerError::StreamSample {
            input_batch_id,
            sample_index,
            ..
        } => {
            assert_eq!(input_batch_id, 7);
            assert_eq!(sample_index, 1);
        }
        other => panic!("unexpected error: {other}"),
    }
    assert_eq!(adapter.stats().errors, 1);
}

#[test]
fn stream_adapter_applies_backpressure_hook() {
    let hook = std::sync::Arc::new(|capacity: QueueCapacity| {
        assert!(capacity.is_full());
        BackpressureAction::Stop
    });
    let mut adapter = TokenizerStreamAdapter::new(LengthTokenizer, 1)
        .with_queue_capacity(QueueCapacity {
            capacity: 1,
            len: 1,
        })
        .with_backpressure_hook(hook);
    let input = RawSampleBatch::new(1, vec![sample("a", b"a")], false);

    let error = adapter
        .tokenize_batch(input)
        .expect_err("backpressure stop");

    assert!(matches!(error, TokenizerError::Parallel(_)));
    assert_eq!(adapter.stats().backpressure_events, 1);
}
