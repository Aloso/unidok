use asciimath_rs::format::mathml::ToMathML;
use itertools::Itertools;
use unidoc_parser::html::ElemName;
use unidoc_parser::inlines::Formatting;
use unidoc_parser::ir::*;

use crate::{Attr, Element, IntoNode, IntoNodes, Node, ToPlaintext};

impl<'a> IntoNode<'a> for SegmentIr<'a> {
    fn into_node(self) -> Node<'a> {
        match self {
            SegmentIr::Text(t) => Node::Text(t),
            SegmentIr::Text2(t) => Node::Text2(t),
            SegmentIr::EscapedText(t) => Node::Text(t),
            SegmentIr::LineBreak => Node::Text("\n"),
            SegmentIr::Limiter => Node::Fragment(vec![]),
            SegmentIr::HtmlEntity(e) => Node::Entity(e.0),
            SegmentIr::Braces(b) => b.into_node(),
            SegmentIr::Link(l) => l.into_node(),
            SegmentIr::Image(i) => i.into_node(),
            SegmentIr::InlineHtml(h) => h.into_node(),
            SegmentIr::Format(f) => f.into_node(),
            SegmentIr::Code(c) => c.into_node(),
            SegmentIr::InlineMacro(m) => m.into_node(),
            SegmentIr::Math(m) => m.into_node(),
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
    fn into_node(mut self) -> Node<'a> {
        match self.href {
            Some(href) => {
                let href = Attr { key: "href", value: Some(href) };
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
            None if self.text.len() == 1 => self.text.pop().unwrap().into_node(),
            None => panic!(
                "Wrong text segment length in unresolved link reference: Expected 1, got {}",
                self.text.len()
            ),
        }
    }
}

impl<'a> IntoNode<'a> for ImageIr<'a> {
    fn into_node(mut self) -> Node<'a> {
        match self.href {
            Some(href) => {
                let mut buf = String::new();
                for a in &self.alt {
                    a.to_plaintext(&mut buf);
                }

                let src = Attr { key: "src", value: Some(href) };
                let alt = Attr { key: "alt", value: Some(buf) };

                Node::Element(Element {
                    name: ElemName::Img,
                    attrs: vec![src, alt],
                    content: None,
                    is_block_level: false,
                    contains_blocks: false,
                })
            }
            None if self.alt.len() == 1 => self.alt.pop().unwrap().into_node(),
            None => panic!(
                "Wrong text segment length in unresolved image link reference: Expected 1, got {}",
                self.alt.len()
            ),
        }
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
            "" => {
                let node = self.segment.into_node();

                if let Node::Element(mut elem) = node {
                    add_attributes(self.args, &mut elem);
                    Node::Element(elem)
                } else {
                    node
                }
            }
            _ => self.segment.into_node(),
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
                    add_attribute_kv(&mut elem.attrs, key, word);
                }
                TokenTreeIr::KV(key, TokenTreeAtomIr::QuotedWord(word)) => {
                    add_attribute_kv(&mut elem.attrs, key, word);
                }
                _ => {}
            }
        }

        if !classes.is_empty() {
            if classes.len() == 1 {
                add_attribute_kv(&mut elem.attrs, "class", classes.pop().unwrap());
            } else {
                add_attribute_kv(&mut elem.attrs, "class", classes.into_iter().join(" "));
            }
        }
    }
}

fn add_attribute_kv<'a>(
    attrs: &mut Vec<Attr<'a>>,
    key: &'a str,
    value: impl ToString + AsRef<str>,
) {
    match key {
        "class" => {
            if let Some(c) = attrs.iter_mut().find(|a| a.key == "class") {
                let old_value = c.value.get_or_insert_with(String::new);
                old_value.push(' ');
                old_value.push_str(value.as_ref());
                return;
            }
        }
        "style" => {
            if let Some(c) = attrs.iter_mut().find(|a| a.key == "style") {
                let old_value = c.value.get_or_insert_with(String::new);
                if !matches!(old_value.trim_end().chars().last(), Some(';') | None) {
                    old_value.push(';');
                }
                old_value.push_str(value.as_ref());
                return;
            }
        }
        _ => {}
    }
    attrs.push(Attr { key, value: Some(value.to_string()) });
}

impl<'a> IntoNode<'a> for MathIr {
    fn into_node(self) -> Node<'a> {
        let formatted = asciimath_rs::parse(self.text).to_mathml();

        Node::Element(Element {
            name: ElemName::Math,
            attrs: vec![],
            content: Some(vec![Node::Verbatim(formatted)]),
            is_block_level: false,
            contains_blocks: true,
        })
    }
}
