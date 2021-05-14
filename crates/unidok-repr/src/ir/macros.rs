#[derive(Debug, Clone, PartialEq)]
pub enum MacroIr<'a> {
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

    Invalid,
}

impl MacroIr<'_> {
    pub fn is_for_list(&self) -> bool {
        matches!(self, MacroIr::Loose | MacroIr::ListStyle(_))
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
