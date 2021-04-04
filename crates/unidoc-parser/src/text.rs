use crate::{Input, Parse, StrSlice};

use crate::indent::Indents;
use crate::NodeParentKind;

pub struct ParseText<'a> {
    pub ind: Indents<'a>,
    pub parent: NodeParentKind,
}

impl Parse for ParseText<'_> {
    type Output = StrSlice;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        fn is_special(c: char) -> bool {
            matches!(
                c,
                '*' | '_' | '~' | '^' | '`' | // inline
                '/' | '\\' | '$' | '&' | // comments, escape, limiter, HTML entity
                '#' | '-' | '.' | ',' | '>' | '<' | '%' | '=' | '[' | '@' |
                '\n' | '{' | '}'
            )
        }

        if input.is_empty() {
            return None;
        }

        match input.rest().find(is_special) {
            Some(0) => None,
            Some(n) => Some(input.bump(n)),
            None => Some(input.bump(input.rest().len())),
        }
    }
}
