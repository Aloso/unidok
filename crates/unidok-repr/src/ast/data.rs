use std::collections::HashMap;

use super::blocks::{HeadingAst, LinkRefDef};
use super::segments::LinkAst;
use crate::config::Config;

#[derive(Debug, Clone, PartialEq)]
pub struct AstData {
    pub link_ref_defs: HashMap<String, LinkRefDef>,
    pub headings: Vec<HeadingAst>,
    pub contains_math: bool,

    pub footnotes: Vec<LinkAst>,
    pub next_footnote: u32,
    pub next_footnote_def: u32,

    pub config: Config,
}

impl AstData {
    pub fn new(config: Config) -> Self {
        AstData {
            link_ref_defs: HashMap::new(),
            headings: Vec::new(),
            contains_math: false,
            footnotes: Vec::new(),
            next_footnote: 1,
            next_footnote_def: 1,
            config,
        }
    }
}

impl Default for AstData {
    fn default() -> Self {
        AstData::new(Config::default())
    }
}
