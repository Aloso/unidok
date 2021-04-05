use crate::marker::ParseLineStart;
use crate::{Input, Parse};

/// The escape character, `\`.
#[derive(Debug, Clone)]
pub struct Escaped {
    pub line_start: bool,
}

impl Escaped {
    pub fn parser() -> ParseEscape {
        ParseEscape
    }
}

pub struct ParseEscape;

impl Parse for ParseEscape {
    type Output = Escaped;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let line_start = input.parse(ParseLineStart).is_some();
        input.parse('\\')?;
        Some(Escaped { line_start })
    }
}
