use crate::ast::html::{ElemClose, ElemName};
use crate::ir::blocks::AnnBlockIr;
use crate::ir::segments::SegmentIr;

use super::blocks::AnnotationIr;

#[derive(Debug, Clone, PartialEq)]
pub enum HtmlNodeIr<'a> {
    Element(HtmlElemIr<'a>),
    CData(&'a str),
    Comment(String),
    Doctype(&'a str),
}

#[derive(Debug, Clone, PartialEq)]
pub struct HtmlElemIr<'a> {
    pub annotations: Vec<AnnotationIr<'a>>,
    pub name: ElemName,
    pub attrs: Vec<AttrIr<'a>>,
    pub content: Option<ElemContentIr<'a>>,
    pub close: ElemClose,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttrIr<'a> {
    pub key: &'a str,
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElemContentIr<'a> {
    Blocks(Vec<AnnBlockIr<'a>>),
    Inline(Vec<SegmentIr<'a>>),
    Verbatim(String),
}
