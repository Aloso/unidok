use unidok_repr::ir::html::{AttrIr, HtmlElemIr, HtmlNodeIr};
use unidok_repr::ir::IrState;

use super::helpers::elem_content_ir_into_nodes;
use super::macros::apply_post_annotations;
use crate::{Attr, Element, IntoNode, Node};

impl<'a> IntoNode<'a> for HtmlNodeIr<'a> {
    fn into_node(self, state: &IrState<'a>) -> Node<'a> {
        match self {
            HtmlNodeIr::Element(e) => e.into_node(state),
            HtmlNodeIr::CData(c) => Node::Cdata(c),
            HtmlNodeIr::Comment(c) => Node::Comment(c),
            HtmlNodeIr::Doctype(d) => Node::Doctype(d),
        }
    }
}

impl<'a> IntoNode<'a> for HtmlElemIr<'a> {
    fn into_node(self, state: &IrState<'a>) -> Node<'a> {
        let content = self.content.map(|c| elem_content_ir_into_nodes(c, state));
        let contains_blocks =
            content.as_ref().map(|c| c.iter().any(Node::is_block_level)).unwrap_or(false);

        let mut node = Node::Element(Element {
            name: self.name,
            attrs: self.attrs.into_iter().map(From::from).collect(),
            content,
            is_block_level: self.name.is_block_level(),
            contains_blocks,
        });
        apply_post_annotations(self.macros, &mut node, state);
        node
    }
}

impl<'a> From<AttrIr<'a>> for Attr<'a> {
    fn from(attr: AttrIr<'a>) -> Self {
        Attr { key: attr.key, value: attr.value }
    }
}
