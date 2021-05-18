use unidok_repr::ir::html::{HtmlElem, HtmlNode};
use unidok_repr::ir::IrState;

use super::helpers::elem_content_ir_into_nodes;
use super::macros::apply_post_annotations;
use crate::{Element, IntoNode, Node};

impl<'a> IntoNode<'a> for HtmlNode<'a> {
    fn into_node(self, state: &IrState<'a>) -> Node<'a> {
        match self {
            HtmlNode::Element(e) => e.into_node(state),
            HtmlNode::CData(c) => Node::Cdata(c),
            HtmlNode::Comment(c) => Node::Comment(c),
            HtmlNode::Doctype(d) => Node::Doctype(d),
        }
    }
}

impl<'a> IntoNode<'a> for HtmlElem<'a> {
    fn into_node(self, state: &IrState<'a>) -> Node<'a> {
        let content = self.content.map(|c| elem_content_ir_into_nodes(c, state));
        let contains_blocks =
            content.as_ref().map(|c| c.iter().any(Node::is_block_level)).unwrap_or(false);

        let mut node = Node::Element(Element {
            name: self.name,
            attrs: self.attrs,
            content,
            is_block_level: self.name.is_block_level(),
            contains_blocks,
        });
        apply_post_annotations(self.macros, &mut node, state);
        node
    }
}
