use crate::parse::Parse;
use crate::utils::Indents;

use super::{CDataSection, Doctype, ElemName, HtmlComment, HtmlElem};

#[derive(Debug, Clone, PartialEq)]
pub enum HtmlNode {
    Element(HtmlElem),
    ClosingTag(ElemName),
    CData(CDataSection),
    Comment(HtmlComment),
    Doctype(Doctype),
}

impl HtmlNode {
    pub(crate) fn parser(ind: Indents<'_>) -> ParseHtmlNode<'_> {
        ParseHtmlNode { ind }
    }
}

pub(crate) struct ParseHtmlNode<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseHtmlNode<'_> {
    type Output = HtmlNode;

    fn parse(&self, input: &mut crate::input::Input) -> Option<Self::Output> {
        Some(if let Some(elem) = input.parse(HtmlElem::parser(self.ind)) {
            HtmlNode::Element(elem)
        } else if let Some(comment) = input.parse(HtmlComment::parser()) {
            HtmlNode::Comment(comment)
        } else if let Some(doctype) = input.parse(Doctype::parser()) {
            HtmlNode::Doctype(doctype)
        } else if let Some(cdata) = input.parse(CDataSection::parser()) {
            HtmlNode::CData(cdata)
        } else {
            return None;
        })
        // TODO
    }
}
