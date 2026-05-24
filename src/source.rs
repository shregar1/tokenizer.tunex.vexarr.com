//! Config-backed tokenizer source for TuneX runtime integration.

use tunex_core::traits::Tokenizer;

use crate::{
    config::{TokenizerConfig, ValidateTokenizerConfig},
    tokenizer::HfTokenizer,
    TokenizerError, TokenizerResult,
};

/// Builds tokenizer implementations from validated TuneX tokenizer config.
#[derive(Clone, Debug)]
pub struct TokenizerSource {
    config: TokenizerConfig,
}

impl TokenizerSource {
    /// Creates a tokenizer source from config.
    ///
    /// # Errors
    ///
    /// Returns an error when config validation fails.
    pub fn new(config: TokenizerConfig) -> TokenizerResult<Self> {
        let report = config.validate();
        if !report.is_valid() {
            return Err(TokenizerError::InvalidConfig(report.to_string()));
        }

        Ok(Self { config })
    }

    /// Returns source config.
    #[must_use]
    pub const fn config(&self) -> &TokenizerConfig {
        &self.config
    }

    /// Loads the configured tokenizer implementation.
    ///
    /// # Errors
    ///
    /// Returns an error when the configured tokenizer cannot be loaded.
    pub fn load(&self) -> TokenizerResult<HfTokenizer> {
        let path = self.config.tokenizer_path.as_ref().ok_or_else(|| {
            TokenizerError::InvalidConfig(
                "tokenizer_name resolution is not implemented; tokenizer_path is required"
                    .to_owned(),
            )
        })?;

        HfTokenizer::from_file_with_config(path, self.config.clone())
    }

    /// Loads the tokenizer behind the core tokenizer trait object.
    ///
    /// # Errors
    ///
    /// Returns an error when the configured tokenizer cannot be loaded.
    pub fn load_boxed(&self) -> TokenizerResult<Box<dyn Tokenizer>> {
        self.load()
            .map(|tokenizer| Box::new(tokenizer) as Box<dyn Tokenizer>)
    }
}

impl TryFrom<TokenizerConfig> for TokenizerSource {
    type Error = TokenizerError;

    fn try_from(config: TokenizerConfig) -> Result<Self, Self::Error> {
        Self::new(config)
    }
}
