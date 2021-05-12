use crate::ast::AstState;
use crate::ir::blocks::HeadingIr;
use crate::IntoIR;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct IrState<'a> {
    pub headings: Vec<HeadingIr<'a>>,
}

impl<'a> IrState<'a> {
    pub fn new(text: &'a str, state: AstState) -> Self {
        let headings = state.headings.clone();
        IrState { headings: headings.into_ir(text, &state) }
    }
}
