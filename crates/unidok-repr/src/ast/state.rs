use std::collections::HashMap;

use super::blocks::{Heading, LinkRefDef};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct AstState {
    pub link_ref_defs: HashMap<String, LinkRefDef>,
    pub headings: Vec<Heading>,
    pub contains_math: bool,
}
