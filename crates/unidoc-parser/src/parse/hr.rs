use crate::{Input, Parse};

use super::indent::Indents;
use super::marker::{ParseLineEnd, ParseLineStart};

/// A horizontal line, consisting of at least three dashes.
pub struct HorizontalLine {
    pub len: usize,
}

pub struct ParseHorizontalLine<'a> {
    pub ind: Indents<'a>,
}

impl Parse for ParseHorizontalLine<'_> {
    type Output = HorizontalLine;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse(ParseLineStart)?;
        input.parse("---")?;
        let mut len = 3;
        while input.parse('-').is_some() {
            len += 1;
        }
        while input.parse(' ').is_some() {}
        input.parse(ParseLineEnd)?;
        let _ = input.parse('\n');

        input.apply();
        Some(HorizontalLine { len })
    }
}
