use std::mem::replace;

use unidoc_repr::ast::html::ElemName;
use unidoc_repr::ir::blocks::AnnotationIr;
use unidoc_repr::ir::macros::MacroArgsIr;
use unidoc_repr::ir::IrState;

use crate::{Attr, Element, IntoNodes, Node};

use super::segment::add_attributes;

pub(crate) fn apply_post_annotations<'a>(
    annotations: Vec<AnnotationIr<'a>>,
    node: &mut Node<'a>,
    state: &IrState<'a>,
) {
    for ann in annotations {
        match ann.name {
            "" => {
                let taken = replace(node, Node::Text(""));
                *node = add_attributes_to_node(taken, ann.args);
            }
            "TOC" => {
                let content = state
                    .headings
                    .iter()
                    .map(|h| {
                        let content = h.segments.clone().into_nodes(state);
                        let link = Element {
                            name: ElemName::A,
                            attrs: vec![Attr { key: "href", value: Some(format!("#{}", h.slug)) }],
                            content: Some(content),
                            is_block_level: false,
                            contains_blocks: false,
                        };
                        Element {
                            name: ElemName::Li,
                            attrs: vec![],
                            content: Some(vec![Node::Element(link)]),
                            is_block_level: true,
                            contains_blocks: false,
                        }
                    })
                    .map(Node::Element)
                    .collect();

                let toc = Element {
                    name: ElemName::Ul,
                    attrs: vec![Attr { key: "class", value: Some("table-of-contents".into()) }],
                    content: Some(content),
                    is_block_level: true,
                    contains_blocks: true,
                };
                *node = Node::Element(toc);
            }
            _ => {}
        }
    }
}

fn add_attributes_to_node<'a>(node: Node<'a>, args: Option<MacroArgsIr<'a>>) -> Node<'a> {
    if let Node::Element(mut elem) = node {
        add_attributes(args, &mut elem);
        Node::Element(elem)
    } else if let Node::Fragment(mut nodes) = node {
        if nodes.len() == 1 {
            let node = nodes.pop().unwrap();
            add_attributes_to_node(node, args)
        } else {
            let mut elem = Element {
                name: ElemName::Div,
                attrs: vec![],
                content: Some(nodes),
                is_block_level: true,
                contains_blocks: true,
            };
            add_attributes(args, &mut elem);
            Node::Element(elem)
        }
    } else {
        panic!("Empty macro can't be applied to this kind of node");
    }
}
