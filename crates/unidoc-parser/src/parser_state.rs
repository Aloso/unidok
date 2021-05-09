use std::collections::HashMap;

use crate::blocks::LinkRefDef;

#[derive(Debug, Default, Clone)]
pub struct ParserState {
    pub link_ref_defs: HashMap<String, LinkRefDef>,
}
