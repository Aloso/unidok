use crate::{Input, Parse, StrSlice};

#[derive(Debug, Clone)]
pub struct SubstText {
    pub content: StrSlice,
    pub substituted: String,
}

pub struct ParseSubstText;

impl Parse for ParseSubstText {
    type Output = SubstText;

    fn parse(&self, _input: &mut Input) -> Option<Self::Output> {
        None
    }
}
