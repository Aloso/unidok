#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub heading_anchor: HeadingAnchor,
}

impl Default for Config {
    fn default() -> Self {
        Config { heading_anchor: HeadingAnchor::None }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HeadingAnchor {
    None,
    Start,
    End,
}
