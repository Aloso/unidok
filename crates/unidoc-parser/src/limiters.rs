use crate::{Input, Parse};

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limiter;

impl Limiter {
    pub fn parser() -> ParseLimiter {
        ParseLimiter
    }
}

pub struct ParseLimiter;

impl Parse for ParseLimiter {
    type Output = Limiter;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        input.parse('$')?;
        Some(Limiter)
    }
}
