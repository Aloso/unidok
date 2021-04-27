mod into_node;
mod nice_debug;
mod plaintext;

use unidoc_parser::html::ElemName;
use unidoc_parser::ir::*;

pub use crate::into_node::{IntoNode, IntoNodes};
pub use crate::plaintext::ToPlaintext;

pub fn convert(ir: DocIr<'_>) -> Vec<Node<'_>> {
    ir.blocks.into_nodes()
}

pub enum Node<'a> {
    Element(Element<'a>),
    Text(&'a str),
    Text2(String),
    Cdata(&'a str),
    Comment(&'a str),
    Doctype(&'a str),
}

pub struct Element<'a> {
    pub name: ElemName,
    pub attrs: Vec<Attr<'a>>,
    pub content: Option<Vec<Node<'a>>>,
}

#[derive(Debug)]
pub struct Attr<'a> {
    pub key: &'a str,
    pub value: Option<String>,
}
