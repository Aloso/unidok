use crate::{Input, Parse, StrSlice};

use crate::indent::Indents;
use crate::marker::ParseLineStart;
use crate::UntilChar;

/// A comment. It starts with two slashes at must appear directly after a line
/// break.
#[derive(Debug, Clone)]
pub struct Comment {
    pub content: StrSlice,
}

pub struct ParseComment<'a> {
    pub ind: Indents<'a>,
}

impl Parse for ParseComment<'_> {
    type Output = Comment;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        input.parse(ParseLineStart)?;
        input.parse("//")?;
        let content = input.parse(UntilChar(|c| c == '\n'))?;
        Some(Comment { content })
    }
}
