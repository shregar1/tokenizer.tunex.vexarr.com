//! Streaming tokenizer statistics.

/// Counters collected by the dataset-to-tokenizer stream adapter.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TokenizerStreamStats {
    /// Raw input batches consumed.
    pub input_batches: u64,
    /// Tokenized output batches emitted.
    pub output_batches: u64,
    /// Input samples consumed.
    pub input_samples: u64,
    /// Output tokens emitted.
    pub output_tokens: u64,
    /// Partial input batches consumed.
    pub partial_batches: u64,
    /// Backpressure events observed.
    pub backpressure_events: u64,
    /// Tokenization errors observed.
    pub errors: u64,
}

impl TokenizerStreamStats {
    /// Records one input batch.
    pub fn record_input(&mut self, samples: usize, is_partial: bool) {
        self.input_batches = self.input_batches.saturating_add(1);
        self.input_samples = self.input_samples.saturating_add(samples as u64);
        if is_partial {
            self.partial_batches = self.partial_batches.saturating_add(1);
        }
    }

    /// Records one output batch.
    pub fn record_output(&mut self, tokens: usize) {
        self.output_batches = self.output_batches.saturating_add(1);
        self.output_tokens = self.output_tokens.saturating_add(tokens as u64);
    }

    /// Records one backpressure event.
    pub fn record_backpressure(&mut self) {
        self.backpressure_events = self.backpressure_events.saturating_add(1);
    }

    /// Records one error.
    pub fn record_error(&mut self) {
        self.errors = self.errors.saturating_add(1);
    }
}
