use crate::quotes::QuoteStyle;

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub heading_anchor: HeadingAnchor,
    pub quote_style: QuoteStyle,
}

impl Default for Config {
    fn default() -> Self {
        Config { heading_anchor: HeadingAnchor::None, quote_style: QuoteStyle::default() }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HeadingAnchor {
    None,
    Start,
    End,
}
