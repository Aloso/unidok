use crate::ast::html::HtmlEntity;
use crate::ast::segments::Formatting;
use crate::ir::html::HtmlNodeIr;

use super::blocks::AnnotationIr;

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
    pub annotations: Vec<AnnotationIr<'a>>,
    pub segments: Vec<SegmentIr<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MathIr<'a> {
    pub annotations: Vec<AnnotationIr<'a>>,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LinkIr<'a> {
    pub annotations: Vec<AnnotationIr<'a>>,
    pub href: Option<String>,
    pub text: Vec<SegmentIr<'a>>,
    pub title: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImageIr<'a> {
    pub annotations: Vec<AnnotationIr<'a>>,
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
    pub annotations: Vec<AnnotationIr<'a>>,
    pub segments: Vec<SegmentIr<'a>>,
}
