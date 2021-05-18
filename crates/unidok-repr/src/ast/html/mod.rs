mod elem_name;
mod entity;

use detached_str::StrSlice;
pub use elem_name::ElemName;
pub use entity::HtmlEntity;

use crate::ast::blocks::BlockAst;
use crate::ast::segments::SegmentAst;

#[derive(Debug, Clone, PartialEq)]
pub enum HtmlNodeAst {
    Element(HtmlElemAst),
    CData(CDataSectionAst),
    Comment(HtmlCommentAst),
    Doctype(DoctypeAst),
}

#[derive(Debug, Clone, PartialEq)]
pub struct HtmlElemAst {
    pub name: ElemName,
    pub attrs: Vec<AttrAst>,
    pub content: Option<ElemContentAst>,
    pub close: ElemClose,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElemContentAst {
    Blocks(Vec<BlockAst>),
    Inline(Vec<SegmentAst>),
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
pub struct CDataSectionAst {
    pub text: StrSlice,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HtmlCommentAst {
    pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DoctypeAst {
    pub text: StrSlice,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttrAst {
    pub key: StrSlice,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttrQuotes {
    Double,
    Single,
    None,
}
