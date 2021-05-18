#[macro_use]
mod util;

mod filter_for_toc;
mod into_node;
mod nice_debug;
mod to_html;

use unidok_parser::DocIr;
use unidok_repr::ast::html::ElemName;
use unidok_repr::ir::html::Attr;

pub use crate::into_node::{IntoNode, IntoNodes};
pub use crate::to_html::ToHtml;

pub fn convert(ir: DocIr<'_>) -> Vec<Node<'_>> {
    ir.blocks.into_nodes(&ir.state)
}

pub fn to_string(nodes: &[Node<'_>]) -> String {
    let mut buf = String::new();
    nodes.to_html(&mut buf, false);
    buf
}

pub enum Node<'a> {
    Element(Element<'a>),
    Text(&'a str),
    Text2(String),
    Entity(&'static str),
    Verbatim(String),
    Cdata(&'a str),
    Comment(String),
    Doctype(&'a str),
    Fragment(Vec<Node<'a>>),
}

impl Node<'_> {
    pub fn is_block_level(&self) -> bool {
        match self {
            Node::Element(e) => e.is_block_level,
            Node::Fragment(f) => f.iter().any(Node::is_block_level),
            _ => false,
        }
    }

    pub fn is_whitespace(&self) -> bool {
        match self {
            &Node::Text(t) => t.trim_start_matches(|c| matches!(c, ' ' | '\t' | '\n')).is_empty(),
            Node::Text2(t) => t.trim_start_matches(|c| matches!(c, ' ' | '\t' | '\n')).is_empty(),
            _ => false,
        }
    }

    /// Returns `true` if the node is [`Element`].
    pub fn is_element(&self) -> bool {
        matches!(self, Self::Element(..))
    }
}

pub struct Element<'a> {
    pub name: ElemName,
    pub attrs: Vec<Attr<'a>>,
    pub content: Option<Vec<Node<'a>>>,
    pub is_block_level: bool,
    pub contains_blocks: bool,
}
