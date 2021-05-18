use crate::ast::html::{ElemClose, ElemName};
use crate::ir::blocks::AnnBlock;
use crate::ir::segments::Segment;

use super::macros::Macro;

#[derive(Debug, Clone, PartialEq)]
pub enum HtmlNode<'a> {
    Element(HtmlElem<'a>),
    CData(&'a str),
    Comment(String),
    Doctype(&'a str),
}

#[derive(Debug, Clone, PartialEq)]
pub struct HtmlElem<'a> {
    pub macros: Vec<Macro<'a>>,
    pub name: ElemName,
    pub attrs: Vec<Attr<'a>>,
    pub content: Option<ElemContent<'a>>,
    pub close: ElemClose,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attr<'a> {
    pub key: &'a str,
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElemContent<'a> {
    Blocks(Vec<AnnBlock<'a>>),
    Inline(Vec<Segment<'a>>),
    Verbatim(String),
}
