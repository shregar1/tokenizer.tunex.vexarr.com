//! Streaming adapter from dataset raw batches to tokenized batches.

use std::sync::Arc;

use async_trait::async_trait;
use tunex_core::events::{
    EventId, EventKind, EventPublisher, EventSeverity, EventSource, TunexEvent,
};

use crate::{
    stream::{
        BackpressureAction, BackpressureHook, QueueCapacity, RawSampleBatch,
        TokenizedBatchEnvelope, TokenizerStreamStats,
    },
    tokenizer::TunexTokenizer,
    TokenizerError, TokenizerResult,
};

/// Async-compatible tokenization stream interface.
#[async_trait]
pub trait AsyncTokenizationStream {
    /// Tokenizes one raw input batch.
    ///
    /// # Errors
    ///
    /// Returns an error with batch/sample context when tokenization fails.
    async fn tokenize_next(
        &mut self,
        batch: RawSampleBatch,
    ) -> TokenizerResult<TokenizedBatchEnvelope>;
}

/// Dataset-to-tokenizer adapter.
pub struct TokenizerStreamAdapter<T> {
    tokenizer: T,
    output_batch_id: u64,
    expected_batch_size: usize,
    queue_capacity: Option<QueueCapacity>,
    backpressure_hook: Option<Arc<dyn BackpressureHook>>,
    event_publisher: Option<Arc<dyn EventPublisher>>,
    stats: TokenizerStreamStats,
}

impl<T> TokenizerStreamAdapter<T>
where
    T: TunexTokenizer,
{
    /// Creates a stream adapter.
    #[must_use]
    pub fn new(tokenizer: T, expected_batch_size: usize) -> Self {
        Self {
            tokenizer,
            output_batch_id: 0,
            expected_batch_size,
            queue_capacity: None,
            backpressure_hook: None,
            event_publisher: None,
            stats: TokenizerStreamStats::default(),
        }
    }

    /// Sets bounded output queue capacity metadata.
    #[must_use]
    pub const fn with_queue_capacity(mut self, capacity: QueueCapacity) -> Self {
        self.queue_capacity = Some(capacity);
        self
    }

    /// Sets a synchronous backpressure hook.
    #[must_use]
    pub fn with_backpressure_hook(mut self, hook: Arc<dyn BackpressureHook>) -> Self {
        self.backpressure_hook = Some(hook);
        self
    }

    /// Sets a core event publisher.
    #[must_use]
    pub fn with_event_publisher(mut self, publisher: Arc<dyn EventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    /// Returns stream stats.
    #[must_use]
    pub const fn stats(&self) -> TokenizerStreamStats {
        self.stats
    }

    /// Tokenizes one raw input batch synchronously.
    ///
    /// # Errors
    ///
    /// Returns an error with batch/sample context when tokenization fails.
    pub fn tokenize_batch(
        &mut self,
        input: RawSampleBatch,
    ) -> TokenizerResult<TokenizedBatchEnvelope> {
        self.apply_backpressure()?;
        self.stats.record_input(
            input.len(),
            input.partial || input.len() < self.expected_batch_size,
        );

        let input_batch_id = input.batch_id;
        let batch = match self.tokenizer.tokenize_batch(&input.samples) {
            Ok(batch) => batch,
            Err(error) => return self.find_sample_error(input_batch_id, &input.samples, error),
        };

        self.stats.record_output(batch.metadata.tokens);
        let output_batch_id = self.output_batch_id;
        self.output_batch_id = self.output_batch_id.saturating_add(1);
        self.emit_event(EventKind::BatchProcessed, input_batch_id, output_batch_id);

        Ok(TokenizedBatchEnvelope {
            input_batch_id,
            output_batch_id,
            batch,
            stats: self.stats,
        })
    }

    /// Tokenizes a sequence of input batches synchronously.
    ///
    /// # Errors
    ///
    /// Returns an error with batch/sample context when tokenization fails.
    pub fn tokenize_batches<I>(&mut self, inputs: I) -> TokenizerResult<Vec<TokenizedBatchEnvelope>>
    where
        I: IntoIterator<Item = RawSampleBatch>,
    {
        inputs
            .into_iter()
            .map(|batch| self.tokenize_batch(batch))
            .collect()
    }

    fn apply_backpressure(&mut self) -> TokenizerResult<()> {
        let Some(capacity) = self.queue_capacity else {
            return Ok(());
        };

        if capacity.is_full() {
            self.stats.record_backpressure();
            self.emit_event(EventKind::BackpressureApplied, 0, self.output_batch_id);
        }

        let Some(hook) = &self.backpressure_hook else {
            return Ok(());
        };

        match hook.on_capacity(capacity) {
            BackpressureAction::Continue | BackpressureAction::Yield => Ok(()),
            BackpressureAction::Stop => Err(TokenizerError::Parallel(
                "stream stopped by backpressure hook".to_owned(),
            )),
        }
    }

    fn find_sample_error(
        &mut self,
        input_batch_id: u64,
        samples: &[tunex_core::types::RawSample],
        batch_error: TokenizerError,
    ) -> TokenizerResult<TokenizedBatchEnvelope> {
        for (sample_index, sample) in samples.iter().enumerate() {
            if let Err(source) = self.tokenizer.tokenize_sample(sample) {
                self.stats.record_error();
                self.emit_event(EventKind::StageFailed, input_batch_id, self.output_batch_id);
                return Err(TokenizerError::StreamSample {
                    input_batch_id,
                    sample_index,
                    source: Box::new(source),
                });
            }
        }

        self.stats.record_error();
        self.emit_event(EventKind::StageFailed, input_batch_id, self.output_batch_id);
        Err(TokenizerError::StreamBatch {
            input_batch_id,
            source: Box::new(batch_error),
        })
    }

    fn emit_event(&self, kind: EventKind, input_batch_id: u64, output_batch_id: u64) {
        let Some(publisher) = &self.event_publisher else {
            return;
        };

        let event = TunexEvent::new(
            EventId::new(format!(
                "tokenizer-stream-{input_batch_id}-{output_batch_id}"
            )),
            kind,
            EventSeverity::Info,
        )
        .with_source(
            EventSource::new()
                .with_module("tunex-tokenizer")
                .with_stage("tokenizer")
                .with_batch(input_batch_id.to_string()),
        )
        .with_metadata("input_batch_id", input_batch_id.to_string())
        .with_metadata("output_batch_id", output_batch_id.to_string());

        let _ = publisher.try_publish(event);
    }
}

#[async_trait]
impl<T> AsyncTokenizationStream for TokenizerStreamAdapter<T>
where
    T: TunexTokenizer + Send,
{
    async fn tokenize_next(
        &mut self,
        batch: RawSampleBatch,
    ) -> TokenizerResult<TokenizedBatchEnvelope> {
        self.tokenize_batch(batch)
    }
}
