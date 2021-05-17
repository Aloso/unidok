use crate::ast::html::HtmlEntity;
use crate::ast::segments::Formatting;
use crate::ir::html::HtmlNodeIr;

use super::macros::MacroIr;

#[derive(Debug, Clone, PartialEq)]
pub enum SegmentIr<'a> {
    LineBreak,
    Text(&'a str),
    Text2(String),
    EscapedText(&'a str),
    Limiter,
    Braces(BracesIr<'a>),
    Math(MathIr<'a>),
    Link(LinkIr<'a>),
    Image(ImageIr<'a>),
    InlineHtml(HtmlNodeIr<'a>),
    HtmlEntity(HtmlEntity),
    Format(InlineFormatIr<'a>),
    Code(CodeIr<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BracesIr<'a> {
    pub macros: Vec<MacroIr<'a>>,
    pub segments: Vec<SegmentIr<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MathIr<'a> {
    pub macros: Vec<MacroIr<'a>>,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LinkIr<'a> {
    pub macros: Vec<MacroIr<'a>>,
    pub href: Option<String>,
    pub text: Vec<SegmentIr<'a>>,
    pub title: Option<String>,
    pub name: Option<String>,
    pub is_superscript: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImageIr<'a> {
    pub macros: Vec<MacroIr<'a>>,
    pub href: Option<String>,
    pub alt: Vec<SegmentIr<'a>>,
    pub title: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InlineFormatIr<'a> {
    pub formatting: Formatting,
    pub segments: Vec<SegmentIr<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CodeIr<'a> {
    pub macros: Vec<MacroIr<'a>>,
    pub segments: Vec<SegmentIr<'a>>,
}
