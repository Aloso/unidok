use detached_str::StrSlice;

use crate::ast::html::{HtmlEntity, HtmlNodeAst};
use crate::ast::macros::InlineMacroAst;

#[derive(Debug, Clone, PartialEq)]
pub enum SegmentAst {
    LineBreak,
    Text(StrSlice),
    Text2(&'static str),
    Text3(String),
    Escaped(Escaped),
    Substitution(Substitution),
    Limiter,
    Braces(BracesAst),
    Math(MathAst),
    Link(LinkAst),
    Image(ImageAst),
    InlineMacro(InlineMacroAst),
    InlineHtml(HtmlNodeAst),
    HtmlEntity(HtmlEntity),
    Format(InlineFormatAst),
    Code(CodeAst),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Escaped {
    pub text: StrSlice,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Substitution {
    Text(&'static str),
    OpenDoubleQuote,
    OpenSingleQuote,
    CloseDoubleQuote,
    CloseSingleQuote,
    Apostrophe,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BracesAst {
    pub segments: Vec<SegmentAst>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MathAst {
    pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LinkAst {
    pub text: Option<Vec<SegmentAst>>,
    pub target: LinkTarget,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImageAst {
    pub alt: Option<Vec<SegmentAst>>,
    pub target: LinkTarget,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LinkTarget {
    Url { href: String, title: Option<String> },
    Reference(StrSlice),
    Footnote,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InlineFormatAst {
    pub formatting: Formatting,
    pub segments: Vec<SegmentAst>,
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
pub struct CodeAst {
    pub segments: Vec<SegmentAst>,
}
