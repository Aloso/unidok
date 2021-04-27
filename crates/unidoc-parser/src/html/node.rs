use crate::parse::Parse;
use crate::utils::Indents;
use crate::StrSlice;

use super::{ElemName, Element};

#[derive(Debug, Clone, PartialEq)]
pub enum HtmlNode {
    Element(Element),
    ClosingTag(ElemName),
    Cdata(StrSlice),
    Comment(StrSlice),
    Doctype(StrSlice),
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
        // TODO
        Some(HtmlNode::Element(input.parse(Element::parser(self.ind))?))
    }
}
