#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;

use crate::quotes::QuoteStyle;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Config {
    pub heading_anchor: HeadingAnchor,
    pub quote_style: QuoteStyle,
    pub retrieve_spans: bool,

    pub unsafe_config: Option<UnsafeConfig>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct UnsafeConfig {
    #[cfg(not(target_arch = "wasm32"))]
    pub root: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HeadingAnchor {
    None,
    Start,
    End,
}

impl Default for HeadingAnchor {
    fn default() -> Self {
        HeadingAnchor::None
    }
}
