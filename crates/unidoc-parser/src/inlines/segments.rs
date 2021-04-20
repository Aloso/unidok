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
    Macro(Macro),
    Format(InlineFormat),
    Code(Code),
}
