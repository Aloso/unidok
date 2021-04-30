use crate::utils::{Indents, ParseLineBreak, ParseSpaces, Until};
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

        input.parse_i(ParseSpaces);
        input.parse("//")?;
        let content = input.parse_i(Until(|c| matches!(c, '\n' | '\r')));
        input.try_parse(ParseLineBreak(self.ind));

        input.apply();
        Some(Comment { content })
    }

    fn can_parse(&self, input: &mut Input) -> bool {
        input.rest().trim_start_matches(|c| matches!(c, ' ' | '\t')).starts_with("//")
    }
}
