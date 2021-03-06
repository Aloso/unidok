use crate::utils::is_ws;
use crate::{Input, Parse};

use super::ParseSpaces;

/// This parser matches if the next character is a line break. The line break is
/// not consumed.
pub(crate) struct ParseLineEnd;

/// This parser matches if the next character (skipping spaces and tabs) is a
/// line break. Only the spaces and tabs are consumed.
pub(crate) struct ParseWsAndLineEnd;

impl Parse for ParseLineEnd {
    type Output = ();

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        match input.peek_char() {
            Some('\n' | '\r') | None => Some(()),
            _ => None,
        }
    }

    fn can_parse(&mut self, input: &mut Input) -> bool {
        matches!(input.peek_char(), Some('\n' | '\r') | None)
    }
}

impl Parse for ParseWsAndLineEnd {
    type Output = ();

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();
        input.parse_i(ParseSpaces);
        match input.peek_char() {
            Some('\n' | '\r') | None => {
                input.apply();
                Some(())
            }
            _ => None,
        }
    }

    fn can_parse(&mut self, input: &mut Input) -> bool {
        matches!(input.rest().trim_start_matches(is_ws).chars().next(), Some('\n' | '\r') | None)
    }
}
