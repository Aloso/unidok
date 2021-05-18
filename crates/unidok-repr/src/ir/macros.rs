use super::segments::Segment;

#[derive(Debug, Clone, PartialEq)]
pub enum Macro<'a> {
    /// `@()`
    HtmlAttrs(Vec<Attr<'a>>),
    /// `@LOOSE`
    Loose,
    /// `@BULLET()`
    ListStyle(String),
    /// `@TOC{}`
    Toc,
    /// `@NOTOC`
    NoToc,
    /// `@NOTXT`
    NoText,
    /// `@FOOTNOTES{}`
    Footnotes(Vec<Footnote<'a>>),
    /// `@MATH_SCRIPT{}`
    MathScript,

    Invalid,
}

impl Macro<'_> {
    pub fn is_for_list(&self) -> bool {
        matches!(self, Macro::Loose | Macro::ListStyle(_))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attr<'a> {
    pub key: &'a str,
    pub value: Option<AttrValue<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttrValue<'a> {
    Word(&'a str),
    QuotedWord(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Footnote<'a> {
    pub num: u32,
    pub text: Vec<Segment<'a>>,
}
