use super::*;
use crate::StrSlice;

#[derive(Debug, Clone, PartialEq)]
pub enum Segment {
    LineBreak(LineBreak),
    Text(StrSlice),
    Text2(&'static str),
    Escaped(Escaped),
    Limiter(Limiter),
    Braces(Braces),
    Math(Math),
    Link(Link),
    Image(Image),
    InlineMacro(InlineMacro),
    Format(InlineFormat),
    Code(Code),
}

impl Default for Segment {
    fn default() -> Self {
        Segment::Text2("")
    }
}
