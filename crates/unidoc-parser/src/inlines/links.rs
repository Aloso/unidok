use crate::str::StrSlice;
use crate::utils::Indents;
use crate::{Input, Parse, UntilChar};

use super::{Segment, SegmentCtx};

#[derive(Debug, Clone, PartialEq)]
pub struct Link {
    pub href: StrSlice,
    pub text: Option<Vec<Segment>>,
}

impl Link {
    pub fn parser(ind: Indents<'_>) -> ParseLink<'_> {
        ParseLink { ind }
    }
}

pub struct ParseLink<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseLink<'_> {
    type Output = Link;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse('<')?;
        let href = input.parse(UntilChar(|c| c == ' ' || c == '\n' || c == '>'))?;
        let text = if input.parse(' ').is_some() || input.parse('\n').is_some() {
            let parser = Segment::multi_parser(SegmentCtx::LinkOrImg, self.ind);
            Some(input.parse(parser)?)
        } else {
            None
        };
        input.parse('>')?;

        input.apply();
        Some(Link { href, text })
    }
}
