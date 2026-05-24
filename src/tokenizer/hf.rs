//! Hugging Face tokenizer integration.

use std::{path::Path, sync::Mutex};

use async_trait::async_trait;
use tokenizers::{PaddingParams, PaddingStrategy, Tokenizer as HfInnerTokenizer, TruncationParams};
use tunex_core::{
    traits::Tokenizer,
    types::{RawSample, TokenizedSample},
    TunexResult,
};

use crate::{
    config::TokenizerConfig,
    tokenizer::{EncodingAdapter, SpecialTokenConfig},
    TokenizedBatch, TokenizerError, TokenizerResult,
};

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

    /// Loads a Hugging Face tokenizer JSON file with explicit TuneX config.
    ///
    /// # Errors
    ///
    /// Returns an error when the tokenizer cannot be loaded or configured.
    pub fn from_file_with_config(
        path: impl AsRef<Path>,
        mut config: TokenizerConfig,
    ) -> TokenizerResult<Self> {
        let path = path.as_ref();
        let inner = HfInnerTokenizer::from_file(path)
            .map_err(|error| TokenizerError::Hf(error.to_string()))?;
        config.tokenizer_path = Some(path.to_path_buf());
        Ok(Self::new(inner, config))
    }

    /// Loads a Hugging Face tokenizer from TuneX config.
    ///
    /// # Errors
    ///
    /// Returns an error when `tokenizer_path` is missing or the tokenizer
    /// cannot be loaded.
    pub fn from_config(config: TokenizerConfig) -> TokenizerResult<Self> {
        let path = config.tokenizer_path.clone().ok_or_else(|| {
            TokenizerError::InvalidConfig(
                "tokenizer_path is required for HfTokenizer::from_config".to_owned(),
            )
        })?;
        Self::from_file_with_config(path, config)
    }

    /// Creates a wrapper from an existing tokenizer.
    pub fn new(mut inner: HfInnerTokenizer, config: TokenizerConfig) -> Self {
        configure_hf_tokenizer(&mut inner, &config);
        Self {
            inner,
            config,
            next_batch_id: Mutex::new(0),
        }
    }

    /// Returns tokenizer config.
    #[must_use]
    pub const fn config(&self) -> &TokenizerConfig {
        &self.config
    }
}

impl TunexTokenizer for HfTokenizer {
    fn tokenize_sample(&self, sample: &RawSample) -> TokenizerResult<TokenizedSample> {
        self.tokenize_batch(std::slice::from_ref(sample))
            .map(|batch| {
                batch
                    .to_samples()
                    .into_iter()
                    .next()
                    .expect("single sample")
            })
    }

    fn tokenize_batch(&self, samples: &[RawSample]) -> TokenizerResult<TokenizedBatch> {
        let texts = samples
            .iter()
            .map(sample_text)
            .collect::<TokenizerResult<Vec<_>>>()?;
        let encodings = self
            .inner
            .encode_batch(texts, self.config.add_special_tokens)
            .map_err(|error| TokenizerError::Hf(error.to_string()))?;
        let adapter = EncodingAdapter::new(
            SpecialTokenConfig::new(self.config.bos_token_id, self.config.eos_token_id),
            self.config.max_seq_len,
            self.config.truncate,
        );
        let outputs = samples
            .iter()
            .zip(encodings.iter())
            .map(|(sample, encoding)| adapter.convert(sample.id.clone(), encoding))
            .collect();
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

fn sample_text(sample: &RawSample) -> TokenizerResult<&str> {
    std::str::from_utf8(&sample.payload).map_err(|_| TokenizerError::InvalidUtf8)
}

fn configure_hf_tokenizer(tokenizer: &mut HfInnerTokenizer, config: &TokenizerConfig) {
    if config.truncate {
        if let Some(max_seq_len) = config.max_seq_len {
            let _ = tokenizer.with_truncation(Some(TruncationParams {
                max_length: max_seq_len,
                ..TruncationParams::default()
            }));
        }
    } else {
        let _ = tokenizer.with_truncation(None);
    }

    if config.padding {
        tokenizer.with_padding(Some(PaddingParams {
            strategy: config
                .pad_to_length
                .map_or(PaddingStrategy::BatchLongest, PaddingStrategy::Fixed),
            pad_id: config.pad_token_id,
            pad_token: config.pad_token.clone(),
            ..PaddingParams::default()
        }));
    } else {
        tokenizer.with_padding(None);
    }
}
