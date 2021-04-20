use crate::parse::ParseInfallible;
use crate::{Input, Parse, StrSlice};

/// Parses until the given `char` parser matches. The matched char itself isn't
/// included. This parser never fails.
pub struct UntilChar<T>(pub T);

/// Parses while the given `char` parser matches. The first char that doesn't
/// match isn't included. This parser never fails.
pub struct WhileChar<T>(pub T);

/// Parser that returns `Some` if the condition returns `true`, otherwise it
/// returns `false`.
pub struct If(pub bool);

/// Parses either the left or, if that fails, the right parser.
pub struct Or<T, U>(pub T, pub U);

impl<F: Fn(char) -> bool> ParseInfallible for UntilChar<F> {
    type Output = StrSlice;

    #[inline]
    fn parse_infallible(&self, input: &mut Input) -> Self::Output {
        match input.rest().find(&self.0) {
            Some(i) => input.bump(i),
            None => input.bump(input.len()),
        }
    }
}

impl ParseInfallible for UntilChar<char> {
    type Output = StrSlice;

    #[inline]
    fn parse_infallible(&self, input: &mut Input) -> Self::Output {
        match input.rest().find(self.0) {
            Some(i) => input.bump(i),
            None => input.bump(input.len()),
        }
    }
}

impl<F: Fn(char) -> bool> ParseInfallible for WhileChar<F> {
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

impl ParseInfallible for WhileChar<char> {
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

impl Parse for If {
    type Output = ();

    fn parse(&self, _input: &mut Input) -> Option<Self::Output> {
        if self.0 {
            Some(())
        } else {
            None
        }
    }
}

impl<T, U> Parse for Or<T, U>
where
    T: Parse,
    U: Parse<Output = T::Output>,
{
    type Output = T::Output;

    #[inline]
    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        match self.0.parse(input) {
            Some(res) => Some(res),
            None => self.1.parse(input),
        }
    }
}
