use crate::ast::AstState;
use crate::ir::blocks::HeadingIr;
use crate::IntoIR;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct IrState<'a> {
    pub headings: Vec<HeadingIr<'a>>,
    pub contains_math: bool,
}

impl<'a> IrState<'a> {
    pub fn new(text: &'a str, mut state: AstState) -> Self {
        let headings = state.headings.clone().into_ir(text, &mut state);
        IrState { headings, contains_math: state.contains_math }
    }
}
