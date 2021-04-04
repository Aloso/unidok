use crate::{Input, Parse};

use super::marker::ParseLineStart;

/// The escape character, `\`.
pub struct Escape {
    pub line_start: bool,
}

pub struct ParseEscape;

impl Parse for ParseEscape {
    type Output = Escape;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let line_start = input.parse(ParseLineStart).is_some();
        input.parse('\\')?;
        Some(Escape { line_start })
    }
}

pub struct ParseLimiter;
#[non_exhaustive]
pub struct Limiter;

impl Parse for ParseLimiter {
    type Output = Limiter;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        input.parse('$')?;
        Some(Limiter)
    }
}
