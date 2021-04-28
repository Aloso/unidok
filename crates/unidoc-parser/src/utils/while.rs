use crate::input::Input;
use crate::parse::ParseInfallible;
use crate::StrSlice;

/// Parses while the given pattern matches. The first occurrence that doesn't
/// match isn't included. This parser never fails.
pub struct While<T>(pub T);

impl<F: Fn(char) -> bool> ParseInfallible for While<F> {
    type Output = StrSlice;

    fn parse_infallible(&self, input: &mut Input) -> Self::Output {
        let mut input = input.start();
        loop {
            match input.peek_char() {
                Some(c) if self.0(c) => {
                    input.bump(c.len_utf8() as usize);
                }
                _ => break,
            };
        }
        input.apply()
    }
}

impl ParseInfallible for While<char> {
    type Output = StrSlice;

    fn parse_infallible(&self, input: &mut Input) -> Self::Output {
        let mut input = input.start();
        loop {
            match input.peek_char() {
                Some(c) if c == self.0 => {
                    input.bump(c.len_utf8() as usize);
                }
                _ => break,
            };
        }
        input.apply()
    }
}
