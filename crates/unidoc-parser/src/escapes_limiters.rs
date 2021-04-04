use crate::{Input, Parse};

use crate::marker::ParseLineStart;

/// The escape character, `\`.
#[derive(Debug, Clone)]
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

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Limiter;

pub struct ParseLimiter;

impl Parse for ParseLimiter {
    type Output = Limiter;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        input.parse('$')?;
        Some(Limiter)
    }
}
