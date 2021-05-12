use crate::{Input, Parse};

pub(crate) struct ParseLimiter;

impl Parse for ParseLimiter {
    type Output = ();

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        input.parse('$')?;
        Some(())
    }
}
