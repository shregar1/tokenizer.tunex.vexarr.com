//! Tokenizer metrics reporter.

use metrics::{counter, gauge, histogram};

use super::{TokenizerCounters, TokenizerLatency, TokenizerThroughput};

/// Full tokenizer metrics aggregate.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TokenizerMetrics {
    /// Tokenizer counters.
    pub counters: TokenizerCounters,
    /// Tokenizer latency accumulators.
    pub latency: TokenizerLatency,
}

impl TokenizerMetrics {
    /// Returns tokens per second using batch latency.
    #[must_use]
    pub fn tokens_per_second(&self) -> f64 {
        self.throughput().tokens_per_sec
    }

    /// Returns samples per second using batch latency.
    #[must_use]
    pub fn samples_per_second(&self) -> f64 {
        self.throughput().samples_per_sec
    }

    /// Returns throughput from aggregate counters and batch latency.
    #[must_use]
    pub fn throughput(&self) -> TokenizerThroughput {
        TokenizerThroughput::from_counts(
            self.counters.samples,
            self.counters.tokens,
            self.latency.batch,
        )
    }

    /// Returns a reportable snapshot.
    #[must_use]
    pub fn snapshot(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            samples_per_sec: self.samples_per_second(),
            tokens_per_sec: self.tokens_per_second(),
            batch_latency_secs: self.latency.batch.as_secs_f64(),
            template_format_latency_secs: self.latency.template_format.as_secs_f64(),
            tokenizer_encode_latency_secs: self.latency.tokenizer_encode.as_secs_f64(),
            cache_hit_ratio: self.counters.cache_hit_ratio(),
            worker_utilization: self.counters.worker_utilization(),
            queue_wait_secs: self.latency.queue_wait.as_secs_f64(),
            allocation_count: self.counters.allocations,
            average_tokens_per_sample: self.counters.average_tokens_per_sample(),
            truncation_count: self.counters.truncations,
            error_count: self.counters.errors,
        }
    }
}

/// Stable metrics snapshot for reporters and tests.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MetricsSnapshot {
    /// Samples per second.
    pub samples_per_sec: f64,
    /// Tokens per second.
    pub tokens_per_sec: f64,
    /// Batch latency in seconds.
    pub batch_latency_secs: f64,
    /// Template formatting latency in seconds.
    pub template_format_latency_secs: f64,
    /// Tokenizer encode latency in seconds.
    pub tokenizer_encode_latency_secs: f64,
    /// Cache hit ratio.
    pub cache_hit_ratio: f64,
    /// Worker utilization ratio.
    pub worker_utilization: f64,
    /// Queue wait time in seconds.
    pub queue_wait_secs: f64,
    /// Allocation count.
    pub allocation_count: u64,
    /// Average tokens per sample.
    pub average_tokens_per_sample: f64,
    /// Truncation count.
    pub truncation_count: u64,
    /// Error count.
    pub error_count: u64,
}

/// Emits tokenizer metrics to the `metrics` facade.
#[derive(Clone, Copy, Debug, Default)]
pub struct TokenizerMetricsReporter;

impl TokenizerMetricsReporter {
    /// Emits one snapshot.
    pub fn report(snapshot: MetricsSnapshot) {
        gauge!("tunex_tokenizer_samples_per_sec").set(snapshot.samples_per_sec);
        gauge!("tunex_tokenizer_tokens_per_sec").set(snapshot.tokens_per_sec);
        histogram!("tunex_tokenizer_batch_latency_secs").record(snapshot.batch_latency_secs);
        histogram!("tunex_tokenizer_template_format_latency_secs")
            .record(snapshot.template_format_latency_secs);
        histogram!("tunex_tokenizer_encode_latency_secs")
            .record(snapshot.tokenizer_encode_latency_secs);
        gauge!("tunex_tokenizer_cache_hit_ratio").set(snapshot.cache_hit_ratio);
        gauge!("tunex_tokenizer_worker_utilization").set(snapshot.worker_utilization);
        histogram!("tunex_tokenizer_queue_wait_secs").record(snapshot.queue_wait_secs);
        counter!("tunex_tokenizer_allocations_total").increment(snapshot.allocation_count);
        gauge!("tunex_tokenizer_average_tokens_per_sample").set(snapshot.average_tokens_per_sample);
        counter!("tunex_tokenizer_truncations_total").increment(snapshot.truncation_count);
        counter!("tunex_tokenizer_errors_total").increment(snapshot.error_count);
    }
}
