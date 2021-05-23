use aho_corasick::AhoCorasick;
use unidok_repr::ast::html::HtmlNodeAst;

use crate::{Indents, Input, Parse};

use super::cdata::ParseCDataSection;
use super::comment::ParseHtmlComment;
use super::doctype::ParseDoctype;
use super::elem::ParseHtmlElem;

pub(crate) struct ParseHtmlNode<'a> {
    pub ind: Indents<'a>,
    pub ac: &'a AhoCorasick,
}

impl Parse for ParseHtmlNode<'_> {
    type Output = HtmlNodeAst;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        Some(if let Some(elem) = input.parse(ParseHtmlElem { ind: self.ind, ac: self.ac }) {
            HtmlNodeAst::Element(elem)
        } else if let Some(comment) = input.parse(ParseHtmlComment { ind: self.ind }) {
            HtmlNodeAst::Comment(comment)
        } else if let Some(doctype) = input.parse(ParseDoctype) {
            HtmlNodeAst::Doctype(doctype)
        } else if let Some(cdata) = input.parse(ParseCDataSection) {
            HtmlNodeAst::CData(cdata)
        } else {
            return None;
        })
    }
}
