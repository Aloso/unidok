use crate::marker::ParseLineStart;
use crate::str::StrSlice;
use crate::{Input, Parse, UntilChar};

/// A comment. It starts with two slashes at must appear directly after a line
/// break.
#[derive(Debug, Clone)]
pub struct Comment {
    pub content: StrSlice,
}

impl Comment {
    pub fn parser() -> ParseComment {
        ParseComment
    }
}

pub struct ParseComment;

impl Parse for ParseComment {
    type Output = Comment;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        input.parse(ParseLineStart)?;
        input.parse("//")?;
        let content = input.parse(UntilChar('\n'))?;
        Some(Comment { content })
    }
}
