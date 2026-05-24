//! Dataset-to-tokenizer streaming adapter contracts.

pub mod adapter;
pub mod backpressure;
pub mod batch_stream;
pub mod stats;

pub use adapter::{AsyncTokenizationStream, TokenizerStreamAdapter};
pub use backpressure::{BackpressureAction, BackpressureHook, BackpressureState, QueueCapacity};
pub use batch_stream::{RawSampleBatch, TokenizedBatchEnvelope};
pub use stats::TokenizerStreamStats;
