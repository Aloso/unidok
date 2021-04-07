use crate::{Input, Parse};

/// This parser matches if a line break has just been parsed.
pub struct ParseLineStart;

/// This parser matches if the next character is a line break. The line break is
/// not consumed.
pub struct ParseLineEnd;

impl Parse for ParseLineStart {
    type Output = ();

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        if input.is_line_start() {
            Some(())
        } else {
            None
        }
    }
}

impl Parse for ParseLineEnd {
    type Output = ();

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        match input.peek_char() {
            Some('\n') | None => Some(()),
            _ => None,
        }
    }
}
