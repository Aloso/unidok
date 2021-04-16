use crate::{Input, Parse};

#[derive(Debug, Clone)]
pub struct Code {
    content: String,
}

impl Code {
    pub(crate) fn parser() -> ParseCode {
        ParseCode
    }
}

pub(crate) struct ParseCode;

impl Parse for ParseCode {
    type Output = Code;

    fn parse(&self, _input: &mut Input) -> Option<Self::Output> {
        todo!()
    }
}
