use std::ops::{Deref, DerefMut};

use crate::str::{Str, StrSlice};
use crate::Parse;

#[derive(Debug, Clone)]
pub struct Input {
    text: Str,
    idx: usize,
}

impl Input {
    pub fn new(text: impl ToString) -> Self {
        Input { text: text.to_string().into(), idx: 0 }
    }

    #[inline]
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn start(&mut self) -> ModifyInput<'_> {
        let prev_idx = self.idx;
        ModifyInput { input: self, prev_idx }
    }

    pub fn len(&self) -> usize {
        self.text.len() - self.idx
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn rest(&self) -> &str {
        &self.text[self.idx as usize..]
    }

    #[cfg(test)]
    pub fn prev(&self) -> &str {
        &self.text[..self.idx as usize]
    }

    pub fn prev_slice_bytes(&self, bytes: usize) -> StrSlice {
        self.text.get(self.idx - bytes..self.idx)
    }

    pub fn bump(&mut self, bytes: usize) -> StrSlice {
        self.idx += bytes;
        self.text.get(self.idx - bytes..self.idx)
    }

    pub fn peek_char(&self) -> Option<char> {
        self.rest().chars().next()
    }

    pub fn prev_char(&self) -> Option<char> {
        let parsed = &self.text[..self.idx as usize];
        parsed.chars().last()
    }

    /// This parses the specified parser and returns the result. If it fails,
    /// [`None`] is returned. For correctness, the parser should NOT be bumped
    /// if `None` is returned.
    #[must_use]
    pub fn parse<P: Parse>(&mut self, parser: P) -> Option<P::Output> {
        parser.parse(self)
    }

    /// This tries to parse the specified parser. If it doesn't succeed, nothing
    /// happens.
    pub fn try_parse<P: Parse>(&mut self, parser: P) {
        parser.parse(self);
    }

    /// This returns whether the parser can be successfully parsed. For
    /// correctness, the parser should NOT be bumped.
    pub fn can_parse<P: Parse>(&mut self, parser: P) -> bool {
        parser.can_parse(self)
    }
}

pub struct ModifyInput<'a> {
    input: &'a mut Input,
    prev_idx: usize,
}

impl ModifyInput<'_> {
    pub fn apply(mut self) -> StrSlice {
        let prev = self.prev_idx;
        self.prev_idx = self.input.idx;
        self.prev_slice_bytes(self.input.idx - prev)
    }
}

impl Deref for ModifyInput<'_> {
    type Target = Input;

    fn deref(&self) -> &Self::Target {
        self.input
    }
}

impl DerefMut for ModifyInput<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.input
    }
}

impl<'a> AsRef<Input> for ModifyInput<'a> {
    fn as_ref(&self) -> &Input {
        self.input
    }
}

impl<'a> AsMut<Input> for ModifyInput<'a> {
    fn as_mut(&mut self) -> &mut Input {
        self.input
    }
}

impl Drop for ModifyInput<'_> {
    fn drop(&mut self) {
        self.input.idx = self.prev_idx;
    }
}

#[test]
fn test_bump() {
    let mut input = Input::new("abcd");
    assert_eq!(input.rest(), "abcd");
    input.bump(2);
    assert_eq!(input.rest(), "cd");
    input.bump(2);
    assert_eq!(input.rest(), "");
}

#[test]
fn test_modify() {
    let mut input = Input::new("abcdef");
    {
        let mut input2 = input.start();
        input2.bump(1);
        assert_eq!(input2.rest(), "bcdef");
    }
    assert_eq!(input.rest(), "abcdef");

    {
        let mut input3 = input.start();
        input3.bump(1);
        input3.apply();
    }
    assert_eq!(input.rest(), "bcdef");

    {
        let mut input4 = input.start();
        input4.bump(1);
        {
            let mut input5 = input4.start();
            input5.bump(2);
            input5.apply();
        }
    }
    assert_eq!(input.rest(), "bcdef");

    {
        let mut input6 = input.start();
        input6.bump(1);
        {
            let mut input7 = input6.start();
            input7.bump(2);
        }
        input6.apply();
    }
    assert_eq!(input.rest(), "cdef");

    {
        let mut input8 = input.start();
        input8.bump(1);
        {
            let mut input9 = input8.start();
            input9.bump(2);
            input9.apply();
        }
        input8.apply();
    }
    assert_eq!(input.rest(), "f");

    {
        let mut input8 = input.start();
        input8.bump(1);
    }

    assert_eq!(input.prev(), "abcde");
}
