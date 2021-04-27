use super::*;
use crate::html::{ElemName, HtmlNode};
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
    InlineHtml(HtmlNode),
    Format(InlineFormat),
    Code(Code),
}

impl Segment {
    pub fn is_closing_tag_for(&self, name: ElemName) -> bool {
        matches!(*self, Segment::InlineHtml(HtmlNode::ClosingTag(n)) if n == name)
    }
}

impl Default for Segment {
    fn default() -> Self {
        Segment::Text2("")
    }
}
