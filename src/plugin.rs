//! TuneX plugin contract for the tokenizer module.

use tunex_core::plugin::{
    PluginCapability, PluginId, PluginKind, PluginMetadata, PluginName, PluginStatus,
    PluginVersion, TunexPlugin,
};

use crate::capabilities::tokenizer_capabilities;

/// Stable TuneX tokenizer plugin name.
pub const TOKENIZER_PLUGIN_NAME: &str = "tunex-tokenizer-hf";

/// Stable TuneX tokenizer plugin version.
pub const TOKENIZER_PLUGIN_VERSION: PluginVersion = PluginVersion::new(0, 1, 0);

/// Tokenizer plugin metadata provider.
#[derive(Clone, Debug)]
pub struct TokenizerPlugin {
    capabilities: Vec<PluginCapability>,
}

impl TokenizerPlugin {
    /// Creates the default tokenizer plugin descriptor.
    #[must_use]
    pub fn new() -> Self {
        Self {
            capabilities: tokenizer_capabilities(),
        }
    }
}

impl Default for TokenizerPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl TunexPlugin for TokenizerPlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata::new(
            PluginId::new(TOKENIZER_PLUGIN_NAME),
            PluginName::new(TOKENIZER_PLUGIN_NAME),
            TOKENIZER_PLUGIN_VERSION,
            PluginKind::Tokenizer,
            self.capabilities.clone(),
        )
    }

    fn capabilities(&self) -> &[PluginCapability] {
        &self.capabilities
    }

    fn health(&self) -> PluginStatus {
        PluginStatus::Active
    }
}
