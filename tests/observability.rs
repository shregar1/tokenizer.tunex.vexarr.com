use std::time::Duration;

use tunex_tokenizer::{
    batch_tokenize_span, TokenizerCounters, TokenizerLatency, TokenizerMetrics,
    TokenizerMetricsReporter,
};

#[test]
fn tokenizer_metrics_compute_throughput_and_ratios() {
    let mut counters = TokenizerCounters::default();
    counters.record_batch(10, 100);
    counters.record_cache_lookup(true);
    counters.record_cache_lookup(false);
    counters.record_worker_ticks(7, 10);

    let mut latency = TokenizerLatency::default();
    latency.record_batch(Duration::from_secs(2));

    let metrics = TokenizerMetrics { counters, latency };
    let snapshot = metrics.snapshot();

    assert_eq!(snapshot.samples_per_sec, 5.0);
    assert_eq!(snapshot.tokens_per_sec, 50.0);
    assert_eq!(snapshot.cache_hit_ratio, 0.5);
    assert_eq!(snapshot.worker_utilization, 0.7);
    assert_eq!(snapshot.average_tokens_per_sample, 10.0);
}

#[test]
fn metrics_reporter_accepts_snapshot_without_recorder() {
    let metrics = TokenizerMetrics::default();

    TokenizerMetricsReporter::report(metrics.snapshot());
}

#[test]
fn tracing_span_helpers_create_named_spans() {
    let span = batch_tokenize_span(128);

    let _entered = span.enter();
}
