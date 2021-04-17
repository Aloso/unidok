use super::*;
use crate::str::StrSlice;

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

// TODO: Think about braces *WITHOUT* macro, i.e.
//
// {some {text} and %{math} and @{macro content}!}
