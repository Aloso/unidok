use detached_str::StrSlice;

use crate::ast::html::{HtmlEntity, HtmlNode};
use crate::ast::macros::InlineMacro;

#[derive(Debug, Clone, PartialEq)]
pub enum Segment {
    LineBreak,
    Text(StrSlice),
    Text2(&'static str),
    Text3(String),
    Escaped(Escaped),
    Limiter,
    Braces(Braces),
    Math(Math),
    Link(Link),
    Image(Image),
    InlineMacro(InlineMacro),
    InlineHtml(HtmlNode),
    HtmlEntity(HtmlEntity),
    Format(InlineFormat),
    Code(Code),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Escaped {
    pub text: StrSlice,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Braces {
    pub segments: Vec<Segment>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Math {
    pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Link {
    pub text: Option<Vec<Segment>>,
    pub target: LinkTarget,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    pub alt: Option<Vec<Segment>>,
    pub target: LinkTarget,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LinkTarget {
    Url { href: String, title: Option<String> },
    Reference(StrSlice),
}

#[derive(Debug, Clone, PartialEq)]
pub struct InlineFormat {
    pub formatting: Formatting,
    pub segments: Vec<Segment>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Formatting {
    Bold,
    Italic,
    StrikeThrough,
    Superscript,
    Subscript,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Code {
    pub segments: Vec<Segment>,
}
