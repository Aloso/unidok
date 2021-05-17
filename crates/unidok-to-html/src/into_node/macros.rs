use std::cmp::Ordering;
use std::mem::replace;

use unidok_repr::ast::html::ElemName;
use unidok_repr::ir::blocks::HeadingIr;
use unidok_repr::ir::macros::{FootnoteIr, MacroIr};
use unidok_repr::ir::{macros, IrState};
use unidok_repr::try_reduce::{Reduced1, TryReduce};

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

                let toc = elem!(<Ul class="table-of-contents"> { content }
                    is_block_level: true, contains_blocks: true);
                *node = Node::Element(toc);
            }
            MacroIr::MathScript if state.contains_math => {
                let s1 = elem!(<Script src="https://polyfill.io/v3/polyfill.min.js?features=es6">[]
                    is_block_level: true, contains_blocks: false);

                let s2 = elem!(<Script id="MathJax-script" async
                    src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/mml-chtml.js">[]
                    is_block_level: true, contains_blocks: false);

                *node = Node::Fragment(vec![Node::Element(s1), Node::Element(s2)]);
            }
            MacroIr::MathScript => *node = Node::Fragment(vec![]),
            MacroIr::Footnotes(footnotes) => {
                if footnotes.is_empty() {
                    *node = Node::Fragment(vec![]);
                } else {
                    let mut children = Vec::with_capacity(footnotes.len() + 1);

                    children.push(Node::Element(elem!(
                        <Hr class="footnotes-line" /> contains_blocks: false, is_block_level: true
                    )));
                    for FootnoteIr { num, text } in footnotes {
                        children.push(Node::Element(elem!(
                        <Div class="footnote-def">[
                            Node::Element(elem!(
                                <A href={format!("#footnote-ref-{}", num)} id={num.to_string()}>[
                                    Node::Text2(num.to_string())
                                ] contains_blocks: false, is_block_level: false
                            )),
                            Node::Text(". "),
                            Node::Fragment(text.into_nodes(state))
                        ] contains_blocks: false, is_block_level: true
                    )));
                    }

                    *node = Node::Element(elem!(
                        <Div class="footnotes-section">{ children } contains_blocks: true, is_block_level: true
                    ))
                }
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
    } else if let Node::Fragment(nodes) = node {
        let nodes = match nodes.try_reduce1() {
            Reduced1::Zero => vec![],
            Reduced1::One(node) => return add_attributes_to_node(node, args),
            Reduced1::Many(nodes) => nodes,
        };
        let mut elem = Element {
            name: ElemName::Div,
            attrs: vec![],
            content: Some(nodes),
            is_block_level: true,
            contains_blocks: true,
        };
        add_attributes(args, &mut elem);
        Node::Element(elem)
    } else {
        panic!("Empty macro can't be applied to this kind of node");
    }
}
