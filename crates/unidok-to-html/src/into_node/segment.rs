use asciimath_rs::format::mathml::ToMathML;
use unidok_repr::ast::html::ElemName;
use unidok_repr::ast::segments::Formatting;
use unidok_repr::ir::segments::*;
use unidok_repr::ir::{macros, IrState};
use unidok_repr::try_reduce::{Reduced1, TryReduce};
use unidok_repr::ToPlaintext;

use crate::into_node::macros::apply_post_annotations;
use crate::{Attr, Element, IntoNode, IntoNodes, Node};

impl<'a> IntoNode<'a> for Segment<'a> {
    fn into_node(self, state: &IrState<'a>) -> Node<'a> {
        match self {
            Segment::Text(t) => Node::Text(t),
            Segment::Text2(t) => Node::Text2(t),
            Segment::EscapedText(t) => Node::Text(t),
            Segment::LineBreak => Node::Text("\n"),
            Segment::Limiter => Node::Fragment(vec![]),
            Segment::HtmlEntity(e) => Node::Entity(e.0),
            Segment::Braces(b) => b.into_node(state),
            Segment::Link(l) => l.into_node(state),
            Segment::Image(i) => i.into_node(state),
            Segment::InlineHtml(h) => h.into_node(state),
            Segment::Format(f) => f.into_node(state),
            Segment::Code(c) => c.into_node(state),
            Segment::Math(m) => m.into_node(state),
        }
    }
}

impl<'a> IntoNode<'a> for Braces<'a> {
    fn into_node(self, state: &IrState<'a>) -> Node<'a> {
        let mut node = Node::Element(elem!(
            <Span>{ self.segments.into_nodes(state) } is_block_level: false, contains_blocks: false
        ));
        apply_post_annotations(self.macros, &mut node, state);
        remove_redundant_spans(node)
    }
}

fn remove_redundant_spans(node: Node<'_>) -> Node<'_> {
    match node {
        Node::Element(e @ Element { name: ElemName::Span, .. }) => {
            if e.attrs.is_empty() {
                match e.content.map(TryReduce::try_reduce1) {
                    None | Some(Reduced1::Zero) => Node::Fragment(vec![]),
                    Some(Reduced1::One(inner)) => inner,
                    Some(Reduced1::Many(inner)) => {
                        Node::Element(Element { content: Some(inner), ..e })
                    }
                }
            } else {
                let content = match e.content.map(TryReduce::try_reduce1) {
                    None | Some(Reduced1::Zero) => Some(vec![]),
                    Some(Reduced1::One(Node::Element(mut inner))) => {
                        inner.attrs.extend(e.attrs);
                        return Node::Element(inner);
                    }
                    Some(Reduced1::One(inner)) => Some(vec![inner]),
                    Some(Reduced1::Many(inner)) => Some(inner),
                };

                Node::Element(Element { content, ..e })
            }
        }
        Node::Fragment(mut f) if f.len() <= 1 => match f.pop() {
            Some(inner) => inner,
            None => Node::Fragment(vec![]),
        },
        node => node,
    }
}

impl<'a> IntoNode<'a> for Link<'a> {
    fn into_node(self, state: &IrState<'a>) -> Node<'a> {
        match self.href {
            Some(href) => {
                let mut attrs = vec![attr!(href = href)];
                if let Some(title) = self.title {
                    attrs.push(attr!(title = title));
                }
                if let Some(n) = self.footnote {
                    attrs.push(attr!(name = format!("footnote-ref-{}", n)));
                    attrs.push(attr!(class = "footnote"));
                }

                let mut node = Node::Element(elem!(
                    <A {attrs}>{ self.text.into_nodes(state) } is_block_level: false, contains_blocks: false
                ));
                apply_post_annotations(self.macros, &mut node, state);

                if self.footnote.is_some() {
                    Node::Element(elem!(
                        <Sup>[node] is_block_level: false, contains_blocks: false
                    ))
                } else {
                    node
                }
            }
            None => Node::Fragment(self.text.into_nodes(state)),
        }
    }
}

impl<'a> IntoNode<'a> for Image<'a> {
    fn into_node(self, state: &IrState<'a>) -> Node<'a> {
        match self.href {
            Some(href) => {
                let mut buf = String::new();
                for a in &self.alt {
                    a.to_plaintext(&mut buf);
                }

                let mut node = Node::Element(elem!(
                    <Img src={href} alt={buf} /> is_block_level: false, contains_blocks: false
                ));
                apply_post_annotations(self.macros, &mut node, state);
                node
            }
            None => Node::Fragment(self.alt.into_nodes(state)),
        }
    }
}

impl<'a> IntoNode<'a> for InlineFormat<'a> {
    fn into_node(self, state: &IrState<'a>) -> Node<'a> {
        let name = match self.formatting {
            Formatting::Bold => ElemName::Strong,
            Formatting::Italic => ElemName::Em,
            Formatting::StrikeThrough => ElemName::S,
            Formatting::Superscript => ElemName::Sup,
            Formatting::Subscript => ElemName::Sub,
        };

        Node::Element(elem!(
            <{name}>{ self.segments.into_nodes(state) } is_block_level: false, contains_blocks: false
        ))
    }
}

impl<'a> IntoNode<'a> for Code<'a> {
    fn into_node(self, state: &IrState<'a>) -> Node<'a> {
        let mut node = Node::Element(elem!(
            <Code>{ self.segments.into_nodes(state) } is_block_level: false, contains_blocks: false
        ));
        apply_post_annotations(self.macros, &mut node, state);
        node
    }
}

pub(super) fn add_attributes<'a>(args: Vec<macros::Attr<'a>>, elem: &mut Element<'a>) {
    for attr in args {
        if let Some(value) = attr.value {
            match value {
                macros::AttrValue::Word(word) => {
                    add_attribute_kv(&mut elem.attrs, attr.key, word);
                }
                macros::AttrValue::QuotedWord(word) => {
                    add_attribute_kv(&mut elem.attrs, attr.key, word);
                }
            }
        } else {
            elem.attrs.push(Attr { key: attr.key, value: None })
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

impl<'a> IntoNode<'a> for Math<'a> {
    fn into_node(self, _: &IrState) -> Node<'a> {
        let formatted = asciimath_rs::parse(self.text).to_mathml();

        Node::Element(elem!(
            <Math>[Node::Verbatim(formatted)] is_block_level: false, contains_blocks: true
        ))
    }
}
