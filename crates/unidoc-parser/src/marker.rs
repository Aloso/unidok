use crate::str::StrSlice;
use crate::{Input, Parse};

/// This parser matches if a line break has just been parsed.
pub struct ParseLineStart;

/// This parser matches if the next character is a line break. The line break is
/// not consumed.
pub struct ParseLineEnd;

/// Wraps a parser that should parse previously parsed text
pub struct Prev<T>(pub T);

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

impl Parse for Prev<char> {
    type Output = char;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        if input.prev_char() == Some(self.0) {
            Some(self.0)
        } else {
            None
        }
    }
}

impl Parse for Prev<&str> {
    type Output = StrSlice;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        if input.prev().ends_with(self.0) {
            Some(input.prev_slice_bytes(self.0.len()))
        } else {
            None
        }
    }
}

impl<F: Fn(Option<char>) -> bool> Parse for Prev<F> {
    type Output = Option<char>;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let prev_char = input.prev_char();
        if self.0(prev_char) {
            Some(prev_char)
        } else {
            None
        }
    }
}
