use crate::{Input, Parse};

/// This parser matches if the next character is a line break. The line break is
/// not consumed.
pub struct ParseLineEnd;

impl Parse for ParseLineEnd {
    type Output = ();

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        match input.peek_char() {
            Some('\n') | None => Some(()),
            _ => None,
        }
    }
}
