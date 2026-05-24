//! Hugging Face tokenizer integration.

use std::{path::Path, sync::Mutex};

use async_trait::async_trait;
use rayon::prelude::*;
use tokenizers::Tokenizer as HfInnerTokenizer;
use tunex_core::{
    traits::Tokenizer,
    types::{RawSample, TokenizedSample},
    TunexResult,
};

use crate::{config::TokenizerConfig, TokenizedBatch, TokenizerError, TokenizerResult};

/// TuneX tokenizer convenience trait for synchronous batch calls.
pub trait TunexTokenizer: Send + Sync {
    /// Tokenizes one raw sample.
    fn tokenize_sample(&self, sample: &RawSample) -> TokenizerResult<TokenizedSample>;

    /// Tokenizes a batch of raw samples.
    fn tokenize_batch(&self, samples: &[RawSample]) -> TokenizerResult<TokenizedBatch>;
}

/// Hugging Face tokenizer wrapper.
pub struct HfTokenizer {
    inner: HfInnerTokenizer,
    config: TokenizerConfig,
    next_batch_id: Mutex<u64>,
}

impl HfTokenizer {
    /// Loads a Hugging Face tokenizer JSON file.
    ///
    /// # Errors
    ///
    /// Returns an error when the tokenizer cannot be loaded.
    pub fn from_file(path: impl AsRef<Path>) -> TokenizerResult<Self> {
        let path = path.as_ref();
        let inner = HfInnerTokenizer::from_file(path)
            .map_err(|error| TokenizerError::Hf(error.to_string()))?;
        Ok(Self::new(inner, TokenizerConfig::from_path(path)))
    }

    /// Creates a wrapper from an existing tokenizer.
    #[must_use]
    pub fn new(inner: HfInnerTokenizer, config: TokenizerConfig) -> Self {
        Self {
            inner,
            config,
            next_batch_id: Mutex::new(0),
        }
    }
}

impl TunexTokenizer for HfTokenizer {
    fn tokenize_sample(&self, sample: &RawSample) -> TokenizerResult<TokenizedSample> {
        let text = std::str::from_utf8(&sample.payload).map_err(|_| TokenizerError::InvalidUtf8)?;
        let encoding = self
            .inner
            .encode(text, self.config.add_special_tokens)
            .map_err(|error| TokenizerError::Hf(error.to_string()))?;
        let mut token_ids = encoding.get_ids().to_vec();

        if let Some(max_seq_len) = self.config.max_seq_len {
            if self.config.truncate && token_ids.len() > max_seq_len {
                token_ids.truncate(max_seq_len);
            }
        }

        let mut output = TokenizedSample::new(token_ids);
        output.id = sample.id.clone();
        Ok(output)
    }

    fn tokenize_batch(&self, samples: &[RawSample]) -> TokenizerResult<TokenizedBatch> {
        let outputs = samples
            .par_iter()
            .map(|sample| self.tokenize_sample(sample))
            .collect::<TokenizerResult<Vec<_>>>()?;
        let mut batch_id = self
            .next_batch_id
            .lock()
            .map_err(|_| TokenizerError::InvalidConfig("batch id mutex poisoned".to_owned()))?;
        let batch = TokenizedBatch::new(*batch_id, outputs);
        *batch_id = batch_id.saturating_add(1);
        Ok(batch)
    }
}

#[async_trait]
impl Tokenizer for HfTokenizer {
    async fn tokenize(&self, sample: &RawSample) -> TunexResult<TokenizedSample> {
        self.tokenize_sample(sample)
            .map_err(tunex_core::TunexError::from)
    }
}
