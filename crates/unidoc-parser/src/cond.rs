use crate::str::StrSlice;
use crate::{Input, Parse};

/// Parses until the given `char` parser matches. The matched char itself isn't
/// included.
pub struct UntilChar<T>(pub T);

/// Parses while the given `char` parser matches. The first char that doesn't
/// match isn't included.
pub struct WhileChar<T>(pub T);

/// Parser that returns `Some` if the condition returns `true`, otherwise it
/// returns `false`.
pub struct If(pub bool);

impl<F: Fn(char) -> bool> Parse for UntilChar<F> {
    type Output = StrSlice;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();
        loop {
            match input.peek_char() {
                Some(c) if self.0(c) => break,
                Some(c) => {
                    input.bump(c.len_utf8() as usize);
                }
                None => break,
            };
        }
        Some(input.apply())
    }
}

impl Parse for UntilChar<char> {
    type Output = StrSlice;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();
        loop {
            match input.peek_char() {
                Some(c) if c == self.0 => break,
                Some(c) => {
                    input.bump(c.len_utf8() as usize);
                }
                None => break,
            };
        }
        Some(input.apply())
    }
}

impl<F: Fn(char) -> bool> Parse for WhileChar<F> {
    type Output = StrSlice;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();
        loop {
            match input.peek_char() {
                Some(c) if self.0(c) => {
                    input.bump(c.len_utf8() as usize);
                }
                _ => break,
            };
        }
        Some(input.apply())
    }
}

impl Parse for WhileChar<char> {
    type Output = StrSlice;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();
        loop {
            match input.peek_char() {
                Some(c) if c == self.0 => {
                    input.bump(c.len_utf8() as usize);
                }
                _ => break,
            };
        }
        Some(input.apply())
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
