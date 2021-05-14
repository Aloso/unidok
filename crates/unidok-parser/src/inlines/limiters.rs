use crate::utils::ParseWsAndLineEnd;
use crate::{Input, Parse};

pub(crate) struct ParseLimiter {
    pub require_line_end: bool,
}

impl Parse for ParseLimiter {
    type Output = ();

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        if self.require_line_end {
            let mut input = input.start();
            input.parse('$')?;
            input.parse(ParseWsAndLineEnd)?;
            input.apply();
            Some(())
        } else {
            input.parse('$')?;
            Some(())
        }
    }
}
