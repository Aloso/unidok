use std::ops::{Deref, DerefMut};

use crate::{Parse, Str, StrSlice};

#[derive(Debug, Clone)]
pub struct Input {
    text: Str,
    idx: usize,
    is_line_start: bool,
}

impl Input {
    pub fn new(text: impl ToString) -> Self {
        Input { text: text.to_string().into(), idx: 0, is_line_start: true }
    }

    pub fn is_line_start(&self) -> bool {
        self.is_line_start
    }

    pub fn start(&mut self) -> ModifyInput<'_> {
        let (prev_idx, prev_l_s) = (self.idx, self.is_line_start);
        ModifyInput { input: self, prev_idx, prev_l_s }
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

    pub fn prev(&self) -> &str {
        &self.text[..self.idx as usize]
    }

    pub fn prev_slice(&self) -> StrSlice {
        self.text.get(..self.idx)
    }

    pub fn prev_slice_bytes(&self, bytes: usize) -> StrSlice {
        self.text.get(self.idx - bytes..self.idx)
    }

    pub fn bump(&mut self, bytes: usize) -> StrSlice {
        self.idx += bytes;
        self.is_line_start = false;
        self.text.get(self.idx - bytes..self.idx)
    }

    pub fn set_line_start(&mut self, is_line_start: bool) {
        self.is_line_start = is_line_start;
    }

    pub fn peek_char(&self) -> Option<char> {
        self.rest().chars().next()
    }

    pub fn prev_char(&self) -> Option<char> {
        let parsed = &self.text[..self.idx as usize];
        parsed.chars().last()
    }

    pub fn parse<P: Parse>(&mut self, parser: P) -> Option<P::Output> {
        parser.parse(self)
    }
}

pub struct ModifyInput<'a> {
    input: &'a mut Input,
    prev_idx: usize,
    prev_l_s: bool,
}

impl ModifyInput<'_> {
    pub fn parsed_bytes(&self) -> usize {
        self.input.idx - self.prev_idx
    }

    pub fn apply(&mut self) -> StrSlice {
        let prev = self.prev_idx;
        self.prev_idx = self.input.idx;
        self.prev_l_s = self.input.is_line_start;
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
        self.input.is_line_start = self.prev_l_s;
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
fn test_line_start() {
    use crate::indent::LineBreak;

    let mut input = Input::new("abcd\nabcd");
    assert!(input.is_line_start());
    input.bump(4);
    assert!(!input.is_line_start());

    input.parse(LineBreak(Default::default())).unwrap();
    assert!(input.is_line_start());
    input.parse("abcd");
    assert!(!input.is_line_start());
}

#[test]
fn test_modify() {
    let mut input = Input::new("abcdef");
    {
        let mut input2 = input.start();
        input2.bump(1);
        assert!(!input2.is_line_start());
    }
    assert_eq!(input.rest(), "abcdef");
    assert!(input.is_line_start());

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

    input.set_line_start(true);
    {
        let mut input8 = input.start();
        input8.bump(1);
    }
    assert!(input.is_line_start());

    assert_eq!(input.prev(), "abcde");
}
