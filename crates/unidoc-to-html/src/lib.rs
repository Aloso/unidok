mod into_node;
mod nice_debug;
mod to_html;
mod to_plaintext;

use unidoc_parser::html::ElemName;
use unidoc_parser::ir::*;

pub use crate::into_node::{IntoNode, IntoNodes};
pub use crate::to_html::ToHtml;
pub use crate::to_plaintext::ToPlaintext;

pub fn convert(ir: DocIr<'_>) -> Vec<Node<'_>> {
    ir.blocks.into_nodes()
}

pub fn to_html(nodes: &[Node<'_>]) -> String {
    let mut buf = String::new();
    nodes.to_html(&mut buf);
    buf
}

pub enum Node<'a> {
    Element(Element<'a>),
    Text(&'a str),
    Text2(String),
    Verbatim(String),
    Cdata(&'a str),
    Comment(&'a str),
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
}

pub struct Element<'a> {
    pub name: ElemName,
    pub attrs: Vec<Attr<'a>>,
    pub content: Option<Vec<Node<'a>>>,
    pub is_block_level: bool,
    pub contains_blocks: bool,
}

#[derive(Debug)]
pub struct Attr<'a> {
    pub key: &'a str,
    pub value: Option<String>,
}
