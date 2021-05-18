use crate::ast::html::HtmlEntity;
use crate::ast::segments::Formatting;
use crate::ir::html::HtmlNode;

use super::macros::Macro;

#[derive(Debug, Clone, PartialEq)]
pub enum Segment<'a> {
    LineBreak,
    Text(&'a str),
    Text2(String),
    EscapedText(&'a str),
    Limiter,
    Braces(Braces<'a>),
    Math(Math<'a>),
    Link(Link<'a>),
    Image(Image<'a>),
    InlineHtml(HtmlNode<'a>),
    HtmlEntity(HtmlEntity),
    Format(InlineFormat<'a>),
    Code(Code<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Braces<'a> {
    pub macros: Vec<Macro<'a>>,
    pub segments: Vec<Segment<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Math<'a> {
    pub macros: Vec<Macro<'a>>,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Link<'a> {
    pub macros: Vec<Macro<'a>>,
    pub href: Option<String>,
    pub text: Vec<Segment<'a>>,
    pub title: Option<String>,
    pub footnote: Option<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Image<'a> {
    pub macros: Vec<Macro<'a>>,
    pub href: Option<String>,
    pub alt: Vec<Segment<'a>>,
    pub title: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InlineFormat<'a> {
    pub formatting: Formatting,
    pub segments: Vec<Segment<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Code<'a> {
    pub macros: Vec<Macro<'a>>,
    pub segments: Vec<Segment<'a>>,
}
