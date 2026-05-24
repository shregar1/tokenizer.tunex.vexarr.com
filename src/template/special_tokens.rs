//! Text-level special token formatting.

/// Text special-token settings used before tokenization.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextSpecialTokens {
    /// Whether to add BOS text.
    pub add_bos: bool,
    /// Whether to add EOS text.
    pub add_eos: bool,
    /// BOS marker.
    pub bos: String,
    /// EOS marker.
    pub eos: String,
}

impl TextSpecialTokens {
    /// Applies text special tokens.
    #[must_use]
    pub fn apply(&self, text: String) -> String {
        let mut output = String::new();
        if self.add_bos {
            output.push_str(&self.bos);
        }
        output.push_str(&text);
        if self.add_eos {
            output.push_str(&self.eos);
        }
        output
    }
}

impl Default for TextSpecialTokens {
    fn default() -> Self {
        Self {
            add_bos: false,
            add_eos: false,
            bos: "<s>".to_owned(),
            eos: "</s>".to_owned(),
        }
    }
}
