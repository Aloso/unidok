use itertools::Itertools;
use unidoc_parser::html::ElemName;
use unidoc_parser::inlines::Formatting;
use unidoc_parser::ir::*;

use crate::{Attr, Element, IntoNode, IntoNodes, Node, ToPlaintext};

impl<'a> IntoNode<'a> for SegmentIr<'a> {
    fn into_node(self) -> Node<'a> {
        match self {
            SegmentIr::Text(t) => Node::Text(t),
            SegmentIr::EscapedText(t) => Node::Text(t),
            SegmentIr::LineBreak => Node::Text("\n"),
            SegmentIr::Limiter => Node::Fragment(vec![]),
            SegmentIr::Braces(b) => b.into_node(),
            SegmentIr::Link(l) => l.into_node(),
            SegmentIr::Image(i) => i.into_node(),
            SegmentIr::InlineHtml(h) => h.into_node(),
            SegmentIr::Format(f) => f.into_node(),
            SegmentIr::Code(c) => c.into_node(),
            SegmentIr::InlineMacro(m) => m.into_node(),
            SegmentIr::Math(_) => todo!(),
        }
    }
}

impl<'a> IntoNode<'a> for BracesIr<'a> {
    fn into_node(self) -> Node<'a> {
        Node::Element(Element {
            name: ElemName::Span,
            attrs: vec![],
            content: Some((self.segments).into_nodes()),
            is_block_level: false,
            contains_blocks: false,
        })
    }
}

impl<'a> IntoNode<'a> for LinkIr<'a> {
    fn into_node(self) -> Node<'a> {
        let href = Attr { key: "href", value: Some(self.href) };
        let attrs = if let Some(title) = self.title {
            let title = Attr { key: "title", value: Some(title) };
            vec![href, title]
        } else {
            vec![href]
        };

        Node::Element(Element {
            name: ElemName::A,
            attrs,
            content: Some(self.text.into_nodes()),
            is_block_level: false,
            contains_blocks: false,
        })
    }
}

impl<'a> IntoNode<'a> for ImageIr<'a> {
    fn into_node(self) -> Node<'a> {
        let mut buf = String::new();
        for a in &self.alt {
            a.to_plaintext(&mut buf);
        }

        let src = Attr { key: "src", value: Some(self.href) };
        let alt = Attr { key: "alt", value: Some(buf) };

        Node::Element(Element {
            name: ElemName::Img,
            attrs: vec![src, alt],
            content: None,
            is_block_level: false,
            contains_blocks: false,
        })
    }
}

impl<'a> IntoNode<'a> for InlineFormatIr<'a> {
    fn into_node(self) -> Node<'a> {
        let name = match self.formatting {
            Formatting::Bold => ElemName::Strong,
            Formatting::Italic => ElemName::Em,
            Formatting::StrikeThrough => ElemName::S,
            Formatting::Superscript => ElemName::Sup,
            Formatting::Subscript => ElemName::Sub,
        };

        Node::Element(Element {
            name,
            attrs: vec![],
            content: Some(self.segments.into_nodes()),
            is_block_level: false,
            contains_blocks: false,
        })
    }
}

impl<'a> IntoNode<'a> for CodeIr<'a> {
    fn into_node(self) -> Node<'a> {
        Node::Element(Element {
            name: ElemName::Code,
            attrs: vec![],
            content: Some(self.segments.into_nodes()),
            is_block_level: false,
            contains_blocks: false,
        })
    }
}

impl<'a> IntoNode<'a> for InlineMacroIr<'a> {
    fn into_node(self) -> Node<'a> {
        match self.name {
            "PASS" | "NOPASS" => self.segment.into_node(),
            "" => {
                let node = self.segment.into_node();

                if let Node::Element(mut elem) = node {
                    add_attributes(self.args, &mut elem);
                    Node::Element(elem)
                } else {
                    node
                }
            }
            _ => todo!(),
        }
    }
}

pub(super) fn add_attributes<'a>(args: Option<MacroArgsIr<'a>>, elem: &mut Element<'a>) {
    if let Some(MacroArgsIr::TokenTrees(tts)) = args {
        let mut classes = Vec::new();

        for tt in tts {
            match tt {
                TokenTreeIr::Atom(TokenTreeAtomIr::Word(arg)) => {
                    if let Some(arg) = arg.strip_prefix('.') {
                        classes.push(arg);
                    } else if let Some(arg) = arg.strip_prefix('#') {
                        elem.attrs.push(Attr { key: "id", value: Some(arg.into()) })
                    } else {
                        elem.attrs.push(Attr { key: arg, value: None })
                    }
                }
                TokenTreeIr::KV(key, TokenTreeAtomIr::Word(word)) => {
                    elem.attrs.push(Attr { key, value: Some(word.into()) });
                }
                TokenTreeIr::KV(key, TokenTreeAtomIr::QuotedWord(word)) => {
                    elem.attrs.push(Attr { key, value: Some(word) });
                }
                _ => {}
            }
        }

        if !classes.is_empty() {
            let value = Some(classes.into_iter().join(" "));
            elem.attrs.push(Attr { key: "class", value })
        }
    }
}
