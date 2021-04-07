use crate::indent::Indents;
use crate::items::LineBreak;
use crate::str::StrSlice;
use crate::{Input, Parse, UntilChar};

/// A line comment.
///
/// It starts with two slashes and must appear directly after a line break. The
/// line break after the comment is ignored.
#[derive(Debug, Clone)]
pub struct Comment {
    pub content: StrSlice,
}

impl Comment {
    pub fn parser(ind: Indents<'_>) -> ParseComment<'_> {
        ParseComment { ind }
    }
}

pub struct ParseComment<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseComment<'_> {
    type Output = Comment;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        input.parse(Self::LINE_START)?;
        let mut input = input.start();

        input.parse(Self::WS);
        input.parse("//")?;
        let content = input.parse(UntilChar('\n'))?;
        input.parse(LineBreak::parser(self.ind));

        input.apply();
        Some(Comment { content })
    }
}
