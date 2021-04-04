use crate::{Input, StrSlice};

use crate::Parse;

/// Parses until the given `char` parser matches. The matched char itself isn't
/// included.
pub struct UntilChar<T>(pub T);

/// Parses until the given `&str` parser matches. The matched string itself
/// isn't included.
pub struct UntilStr<T>(pub T);

impl Parse for char {
    type Output = char;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        if input.rest().starts_with(*self) {
            input.bump(self.len_utf8() as usize);
            Some(*self)
        } else {
            None
        }
    }
}

impl Parse for &str {
    type Output = StrSlice;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        if input.rest().starts_with(*self) {
            Some(input.bump(self.len()))
        } else {
            None
        }
    }
}

impl Parse for String {
    type Output = StrSlice;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        if input.rest().starts_with(self.as_str()) {
            Some(input.bump(self.len()))
        } else {
            None
        }
    }
}

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

impl Parse for UntilStr<&'_ str> {
    type Output = StrSlice;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        input.rest().find(self.0).map(|n| input.bump(n))
    }
}
