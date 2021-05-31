use crate::ast::AstData;
use crate::config::Config;
use crate::ir::blocks::Heading;
use crate::IntoIR;

use super::segments::Link;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct IrState<'a> {
    pub headings: Vec<Heading<'a>>,
    pub contains_math: bool,

    pub footnotes: Vec<Link<'a>>,
    pub footnote_index: usize,
    pub footnote_number: u32,

    pub config: Config,
}

impl<'a> IrState<'a> {
    pub fn new(text: &'a str, mut state: AstData) -> Self {
        let headings = state.headings.clone().into_ir(text, &mut state);
        let footnotes = state.footnotes.clone().into_ir(text, &mut state);
        let contains_math = state.contains_math;

        IrState {
            headings,
            contains_math,
            footnotes,
            footnote_index: 0,
            footnote_number: 1,
            config: state.config,
        }
    }
}
