use std::cmp::Ordering;
use std::mem::replace;

use unidok_repr::ast::html::ElemName;
use unidok_repr::ir::blocks::HeadingIr;
use unidok_repr::ir::macros::MacroIr;
use unidok_repr::ir::{macros, IrState};

use crate::filter_for_toc::filter_for_toc;
use crate::{Attr, Element, IntoNodes, Node};

use super::segment::add_attributes;

pub(crate) fn apply_post_annotations<'a>(
    macros: Vec<MacroIr<'a>>,
    node: &mut Node<'a>,
    state: &IrState<'a>,
) {
    for r#macro in macros {
        match r#macro {
            MacroIr::HtmlAttrs(attrs) => {
                let taken = replace(node, Node::Text(""));
                *node = add_attributes_to_node(taken, attrs);
            }
            MacroIr::Toc => {
                let first_is_level_1 = state.headings.first().into_iter().any(|h| h.level == 1);
                let rem_has_level_1 = state.headings.iter().skip(1).any(|h| h.level == 1);

                let level = if rem_has_level_1 { 1 } else { 2 };
                let headings = if first_is_level_1 && !rem_has_level_1 {
                    &state.headings[1..]
                } else {
                    &state.headings
                };

                let (content, _) = toc_list(level, headings, state);

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

fn toc_list<'a>(
    level: u8,
    headings: &'_ [HeadingIr<'a>],
    state: &'_ IrState<'a>,
) -> (Vec<Node<'a>>, usize) {
    let mut result = Vec::new();

    let mut i = 0;
    while i < headings.len() {
        let heading = &headings[i];

        match heading.level.cmp(&level) {
            Ordering::Equal => {
                let content = filter_for_toc(&heading.segments).into_nodes(state);
                let link = Element {
                    name: ElemName::A,
                    attrs: vec![Attr { key: "href", value: Some(format!("#{}", heading.slug)) }],
                    content: Some(content),
                    is_block_level: false,
                    contains_blocks: false,
                };
                let li = Element {
                    name: ElemName::Li,
                    attrs: vec![],
                    content: Some(vec![Node::Element(link)]),
                    is_block_level: true,
                    contains_blocks: false,
                };
                result.push(Node::Element(li));

                i += 1;
            }
            Ordering::Greater => {
                let (n, new_i) = toc_list(level + 1, &headings[i..], state);
                i += new_i;

                if result.is_empty() {
                    let li = Element {
                        name: ElemName::Li,
                        attrs: vec![],
                        content: Some(vec![]),
                        is_block_level: true,
                        contains_blocks: true,
                    };
                    result.push(Node::Element(li));
                }

                let last = result.last_mut().unwrap();

                if let Node::Element(Element { content: Some(content), contains_blocks, .. }) = last
                {
                    *contains_blocks = true;
                    for n in &mut *content {
                        if let Node::Element(e) = n {
                            e.is_block_level = true;
                        }
                    }
                    content.push(Node::Element(Element {
                        name: ElemName::Ul,
                        attrs: vec![],
                        content: Some(n),
                        is_block_level: true,
                        contains_blocks: true,
                    }));
                } else {
                    unreachable!("Last node in the TOC list is not a proper element");
                }
            }
            Ordering::Less => {
                return (result, i);
            }
        }
    }

    (result, i)
}

fn add_attributes_to_node<'a>(node: Node<'a>, args: Vec<macros::Attr<'a>>) -> Node<'a> {
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
