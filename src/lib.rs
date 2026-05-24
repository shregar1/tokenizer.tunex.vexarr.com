//! High-performance tokenizer engine for the TuneX ecosystem.
//!
//! The v0.1 surface provides stable tokenizer module contracts, batch output
//! types, Hugging Face tokenizer integration, plugin metadata, and a public
//! prelude. Heavy optimization work is intentionally left for later phases.

pub mod config;
pub mod error;
pub mod metrics;
pub mod plugin;
pub mod prelude;
pub mod tokenizer;

pub use config::TokenizerConfig;
pub use error::{TokenizerError, TokenizerResult};
pub use metrics::TokenizerMetrics;
pub use plugin::{TokenizerPlugin, TOKENIZER_PLUGIN_NAME, TOKENIZER_PLUGIN_VERSION};
pub use tokenizer::{
    BatchTokenizationPlan, BatchTokenizationScheduler, EncodingAdapter, HfTokenizer,
    ParallelBatchTokenizer, ParallelTokenizationStats, SampleChunk, SpecialTokenConfig,
    TokenizationWorker, TokenizedBatch, TokenizedBatchMetadata, TokenizedOutput, TunexTokenizer,
    WorkerTokenizationStats,
};
