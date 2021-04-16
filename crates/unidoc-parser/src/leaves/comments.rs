use crate::str::StrSlice;
use crate::utils::{Indents, ParseLineBreak, ParseSpaces};
use crate::{Input, Parse, UntilChar};

/// A line comment.
///
/// It starts with two slashes and must appear directly after a line break. The
/// line break after the comment is ignored.
#[derive(Debug, Clone, PartialEq)]
pub struct Comment {
    pub content: StrSlice,
}

impl Comment {
    pub(crate) fn parser(ind: Indents<'_>) -> ParseComment<'_> {
        ParseComment { ind }
    }
}

pub(crate) struct ParseComment<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseComment<'_> {
    type Output = Comment;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.try_parse(ParseSpaces);
        input.parse("//")?;
        let content = input.parse(UntilChar('\n'))?;
        input.try_parse(ParseLineBreak(self.ind));

        input.apply();
        Some(Comment { content })
    }

    fn can_parse(&self, input: &mut Input) -> bool {
        input.rest().starts_with("//")
    }
}
