//! Tracing span helpers for tokenizer operations.

use ::tracing::{info_span, Span};

/// Creates a tokenizer load span.
#[must_use]
pub fn tokenizer_load_span(source: &str) -> Span {
    info_span!("tokenizer_load", source)
}

/// Creates a template formatting span.
#[must_use]
pub fn template_format_span(template: &str) -> Span {
    info_span!("template_format", template)
}

/// Creates a batch tokenization span.
#[must_use]
pub fn batch_tokenize_span(samples: usize) -> Span {
    info_span!("batch_tokenize", samples)
}

/// Creates a parallel tokenization span.
#[must_use]
pub fn parallel_tokenize_span(samples: usize, workers: usize) -> Span {
    info_span!("parallel_tokenize", samples, workers)
}

/// Creates a cache lookup span.
#[must_use]
pub fn cache_lookup_span(target: &str) -> Span {
    info_span!("cache_lookup", target)
}

/// Creates a cache store span.
#[must_use]
pub fn cache_store_span(target: &str) -> Span {
    info_span!("cache_store", target)
}

/// Creates a stream consume span.
#[must_use]
pub fn stream_consume_span(input_batch_id: u64) -> Span {
    info_span!("stream_consume", input_batch_id)
}

/// Creates a stream emit span.
#[must_use]
pub fn stream_emit_span(input_batch_id: u64, output_batch_id: u64) -> Span {
    info_span!("stream_emit", input_batch_id, output_batch_id)
}
