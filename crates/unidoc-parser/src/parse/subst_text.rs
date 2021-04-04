use crate::{Input, Parse, StrSlice};

pub struct ParseSubstText;
pub struct SubstText {
    pub content: StrSlice,
    pub substituted: String,
}

impl Parse for ParseSubstText {
    type Output = SubstText;

    fn parse(&self, _input: &mut Input) -> Option<Self::Output> {
        todo!()
    }
}
