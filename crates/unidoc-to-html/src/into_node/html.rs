use unidoc_parser::ir::{AttrIr, HtmlElemIr, HtmlNodeIr};

use super::helpers::elem_content_ir_into_nodes;
use crate::{Attr, Element, IntoNode, Node};

impl<'a> IntoNode<'a> for HtmlNodeIr<'a> {
    fn into_node(self) -> Node<'a> {
        match self {
            HtmlNodeIr::Element(e) => e.into_node(),
            HtmlNodeIr::ClosingTag(_) => Node::Text(""),
            HtmlNodeIr::CData(c) => Node::Cdata(c),
            HtmlNodeIr::Comment(c) => Node::Comment(c),
            HtmlNodeIr::Doctype(d) => Node::Doctype(d),
        }
    }
}

impl<'a> IntoNode<'a> for HtmlElemIr<'a> {
    fn into_node(self) -> Node<'a> {
        let content = self.content.map(elem_content_ir_into_nodes);
        let contains_blocks =
            content.as_ref().map(|c| c.iter().any(Node::is_block_element)).unwrap_or(false);

        Node::Element(Element {
            name: self.name,
            attrs: self.attrs.into_iter().map(From::from).collect(),
            content,
            is_block_level: self.name.is_block_level(),
            contains_blocks,
        })
    }
}

impl<'a> From<AttrIr<'a>> for Attr<'a> {
    fn from(attr: AttrIr<'a>) -> Self {
        Attr { key: attr.key, value: attr.value }
    }
}
