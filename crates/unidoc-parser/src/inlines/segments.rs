use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Segment {
    LineBreak(LineBreak),
    Text(Text),
    Escaped(Escaped),
    Limiter(Limiter),
    Braces(Braces),
    Math(Math),
    Link(Link),
    Image(Image),
    Macro(Macro),
    InlineFormat(InlineFormat),
}

// TODO: Think about braces *WITHOUT* macro, i.e.
//
// {some {text} and %{math} and @{macro content}!}
