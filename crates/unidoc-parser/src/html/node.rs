use unidoc_repr::ast::html::HtmlNode;

use crate::{Indents, Input, Parse};

use super::cdata::ParseCDataSection;
use super::comment::ParseHtmlComment;
use super::doctype::ParseDoctype;
use super::elem::ParseHtmlElem;

pub(crate) struct ParseHtmlNode<'a> {
    pub ind: Indents<'a>,
}

impl Parse for ParseHtmlNode<'_> {
    type Output = HtmlNode;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        Some(if let Some(elem) = input.parse(ParseHtmlElem { ind: self.ind }) {
            HtmlNode::Element(elem)
        } else if let Some(comment) = input.parse(ParseHtmlComment { ind: self.ind }) {
            HtmlNode::Comment(comment)
        } else if let Some(doctype) = input.parse(ParseDoctype) {
            HtmlNode::Doctype(doctype)
        } else if let Some(cdata) = input.parse(ParseCDataSection) {
            HtmlNode::CData(cdata)
        } else {
            return None;
        })
    }
}
