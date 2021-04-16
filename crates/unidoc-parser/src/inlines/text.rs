use std::fmt;

use crate::str::StrSlice;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Text(pub StrSlice);

impl fmt::Debug for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Text @ {}..{}", self.0.start(), self.0.end())
    }
}
