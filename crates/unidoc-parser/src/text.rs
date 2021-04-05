use std::fmt;

use crate::indent::Indents;
use crate::str::StrSlice;
use crate::{Input, Parse};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Text(pub StrSlice);

impl Text {
    pub fn parser(ind: Indents<'_>) -> ParseText<'_> {
        ParseText { ind }
    }
}

impl fmt::Debug for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Text @ {}..{}", self.0.start(), self.0.end())
    }
}

pub struct ParseText<'a> {
    pub ind: Indents<'a>,
}

impl Parse for ParseText<'_> {
    type Output = Text;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        fn is_special(c: char) -> bool {
            matches!(
                c,
                '\n' | // block elements
                '*' | '_' | '~' | '^' | '`' | // inline
                '\\' | '$' | // escape, limiter
                '<' | // links, images
                '%' | // math
                '[' | // attributes
                '@' | // macros
                '{' | '}' | // braces, macro bodies
                '|' // table cells
            )
        }

        if input.is_empty() {
            return None;
        }

        match input.rest().find(is_special) {
            Some(0) => None,
            Some(n) => Some(Text(input.bump(n))),
            None => Some(Text(input.bump(input.rest().len()))),
        }
    }
}
