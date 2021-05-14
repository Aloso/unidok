use unidoc_repr::ast::blocks::Comment;

use crate::utils::{is_ws, ParseSpaces, Until};
use crate::{Input, Parse};

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
        input.rest().trim_start_matches(is_ws).starts_with("//")
    }
}
