//! High-performance tokenizer engine for the TuneX ecosystem.
//!
//! The v0.1 surface provides stable tokenizer module contracts, batch output
//! types, Hugging Face tokenizer integration, plugin metadata, and a public
//! prelude. Heavy optimization work is intentionally left for later phases.

pub mod batch;
pub mod cache;
pub mod capabilities;
pub mod config;
pub mod error;
pub mod metrics;
pub mod plugin;
pub mod prelude;
pub mod source;
pub mod template;
pub mod tokenizer;

pub use batch::{
    TokenBuffer, TokenOffsets, TokenizedBatch, TokenizedBatchBuilder, TokenizedBatchMetadata,
    TokenizedBatchStats, TokenizedBatchView, TokenizedSampleView,
};
pub use cache::{
    CacheEvictionPolicy, CachePolicy, CacheStorage, CacheTarget, DiskTokenizerCache,
    MemoryTokenizerCache, TokenizerCacheEntry, TokenizerCacheKey, TokenizerCacheStats,
    TokenizerFingerprint,
};
pub use capabilities::{
    tokenizer_capabilities, BATCH_TOKENIZATION, CHAT_TEMPLATE, HF_TOKENIZER, PADDING,
    PARALLEL_TOKENIZATION, SPECIAL_TOKENS, TOKENIZER_CAPABILITY_NAMES, TRUNCATION,
};
pub use config::{
    ChatTemplateSelection, ConfigValidationError, ConfigValidationReport, PaddingConfig,
    TemplateConfig, TokenizerConfig, TruncationConfig, ValidateTokenizerConfig,
};
pub use error::{TokenizerError, TokenizerResult};
pub use metrics::TokenizerMetrics;
pub use plugin::{TokenizerPlugin, TOKENIZER_PLUGIN_NAME, TOKENIZER_PLUGIN_VERSION};
pub use source::TokenizerSource;
pub use template::{
    ChatMessage, ChatRole, ChatTemplate, ChatTemplateConfig, TemplateFormatter, TemplateKind,
    TemplateValidationError, TextSpecialTokens,
};
pub use tokenizer::{
    BatchTokenizationPlan, BatchTokenizationScheduler, EncodingAdapter, HfTokenizer,
    ParallelBatchTokenizer, ParallelTokenizationStats, SampleChunk, SpecialTokenConfig,
    TokenizationWorker, TokenizedOutput, TunexTokenizer, WorkerTokenizationStats,
};
