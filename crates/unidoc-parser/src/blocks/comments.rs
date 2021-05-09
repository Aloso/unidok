use crate::utils::{ParseSpaces, Until};
use crate::{Input, Parse, StrSlice};

/// A line comment
///
/// ### Example
///
/// ````md
/// // This is a comment
/// ````
#[derive(Debug, Clone, PartialEq)]
pub struct Comment {
    pub content: StrSlice,
}

impl Comment {
    pub(crate) fn parser() -> ParseComment {
        ParseComment
    }
}

pub(crate) struct ParseComment;

impl Parse for ParseComment {
    type Output = Comment;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse_i(ParseSpaces);
        input.parse("//")?;
        let content = input.parse_i(Until(|c| matches!(c, '\n' | '\r')));

        input.apply();
        Some(Comment { content })
    }

    fn can_parse(&mut self, input: &mut Input) -> bool {
        input.rest().trim_start_matches(|c| matches!(c, ' ' | '\t')).starts_with("//")
    }
}
