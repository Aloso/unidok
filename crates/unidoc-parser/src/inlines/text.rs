use std::fmt;

use crate::str::StrSlice;
use crate::{Input, Parse};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Text(pub StrSlice);

impl Text {
    pub fn parser() -> ParseText {
        ParseText
    }
}

impl fmt::Debug for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Text @ {}..{}", self.0.start(), self.0.end())
    }
}

pub struct ParseText;

impl Parse for ParseText {
    type Output = Text;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        fn is_special(c: char) -> bool {
            matches!(c, '\n') // block comments
                || matches!(c, '*' | '_' | '~' | '^' | '`' | '#') // inline
                || matches!(c, '\\' | '$') // escape, limiter
                || matches!(c, '!' | '[' | ']') // links, images
                || matches!(c, '%' | '@') // math, macros
                || matches!(c, '}' | '|') // macro bodies, table cells
                || matches!(c, '<') // HTML
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
