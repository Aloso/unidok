mod elem_name;
mod entity;

use detached_str::StrSlice;
pub use elem_name::ElemName;
pub use entity::HtmlEntity;

use crate::ast::blocks::Block;
use crate::ast::segments::Segment;

#[derive(Debug, Clone, PartialEq)]
pub enum HtmlNode {
    Element(HtmlElem),
    CData(CDataSection),
    Comment(HtmlComment),
    Doctype(Doctype),
}

#[derive(Debug, Clone, PartialEq)]
pub struct HtmlElem {
    pub name: ElemName,
    pub attrs: Vec<HtmlAttr>,
    pub content: Option<ElemContent>,
    pub close: ElemClose,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElemContent {
    Blocks(Vec<Block>),
    Inline(Vec<Segment>),
    Verbatim(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElemClose {
    /// `<br>`
    AutoSelfClosing,
    /// `<br />`
    SelfClosing,
    /// ```html
    /// <ul>
    ///     <li>Element</li>
    /// </ul>
    /// ```
    Normal,
    /// ```html
    /// <ul>
    ///     <li>Element
    /// </ul>
    /// ```
    AutoClosing,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CDataSection {
    pub text: StrSlice,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HtmlComment {
    pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Doctype {
    pub text: StrSlice,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HtmlAttr {
    pub key: StrSlice,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttrQuotes {
    Double,
    Single,
    None,
}
