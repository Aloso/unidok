use itertools::Itertools;
use unidoc_parser::html::ElemName;
use unidoc_parser::ir::*;

pub fn convert(ir: DocIr<'_>) -> Vec<Node<'_>> {
    ir.blocks.into_iter().map(From::from).collect()
}

#[derive(Debug)]
pub enum Node<'a> {
    Element(Element<'a>),
    Text(&'a str),
    Text2(String),
    Cdata(&'a str),
    Comment(&'a str),
    Doctype(&'a str),
}

#[derive(Debug)]
pub struct Element<'a> {
    pub name: ElemName,
    pub attrs: Vec<Attr<'a>>,
    pub content: Option<Vec<Node<'a>>>,
}

#[derive(Debug)]
pub struct Attr<'a> {
    pub key: &'a str,
    pub value: Option<&'a str>,
}

impl<'a> From<HtmlNodeIr<'a>> for Node<'a> {
    fn from(node: HtmlNodeIr<'a>) -> Self {
        match node {
            HtmlNodeIr::Element(e) => Node::Element(e.into()),
            HtmlNodeIr::ClosingTag(_) => Node::Text(""),
            HtmlNodeIr::Cdata(c) => Node::Cdata(c),
            HtmlNodeIr::Comment(c) => Node::Comment(c),
            HtmlNodeIr::Doctype(d) => Node::Doctype(d),
        }
    }
}

impl<'a> From<HtmlElemIr<'a>> for Element<'a> {
    fn from(elem: HtmlElemIr<'a>) -> Self {
        Element {
            name: elem.name,
            attrs: elem.attrs.into_iter().map(From::from).collect(),
            content: elem.content.map(elem_content_ir_into_nodes),
        }
    }
}

impl<'a> From<AttrIr<'a>> for Attr<'a> {
    fn from(attr: AttrIr<'a>) -> Self {
        Attr { key: attr.key, value: attr.value }
    }
}

fn elem_content_ir_into_nodes(content: ElemContentIr<'_>) -> Vec<Node<'_>> {
    match content {
        ElemContentIr::Blocks(b) => b.into_iter().map(From::from).collect(),
        ElemContentIr::Inline(i) => i.into_iter().map(From::from).collect(),
        ElemContentIr::Verbatim(v) => vec![Node::Text(v)],
    }
}

impl<'a> From<BlockIr<'a>> for Node<'a> {
    fn from(block: BlockIr<'a>) -> Self {
        match block {
            BlockIr::CodeBlock(c) => Node::Element(Element {
                name: ElemName::Pre,
                attrs: if c.info.trim_start() != "" {
                    vec![Attr { key: "data-language", value: Some(c.info) }]
                } else {
                    vec![]
                },
                content: Some(vec![Node::Text2(c.lines.into_iter().join("\n"))]),
            }),
            BlockIr::Comment(_) => Node::Text(""),
            BlockIr::Paragraph(p) => Node::Element(Element {
                name: ElemName::P,
                attrs: vec![],
                content: Some(convert_segments(p.segments)),
            }),
            BlockIr::Heading(h) => Node::Element(Element {
                name: match h.level {
                    1 => ElemName::H1,
                    2 => ElemName::H2,
                    3 => ElemName::H3,
                    4 => ElemName::H4,
                    5 => ElemName::H5,
                    6 => ElemName::H6,
                    l => panic!("Invalid heading level {}", l),
                },
                attrs: vec![],
                content: Some(convert_segments(h.segments)),
            }),
            BlockIr::Table(_) => todo!(),
            BlockIr::ThematicBreak(_) => {
                Node::Element(Element { name: ElemName::Hr, attrs: vec![], content: None })
            }
            BlockIr::List(_) => todo!(),
            BlockIr::Quote(_) => todo!(),
            BlockIr::BlockMacro(_) => todo!(),
            BlockIr::BlockHtml(h) => h.into(),
        }
    }
}

impl<'a> From<SegmentIr<'a>> for Node<'a> {
    fn from(segment: SegmentIr<'a>) -> Self {
        match segment {
            SegmentIr::LineBreak => {
                Node::Element(Element { name: ElemName::Br, attrs: vec![], content: None })
            }
            SegmentIr::Text(t) => Node::Text(t),
            SegmentIr::EscapedText(t) => Node::Text(t),
            SegmentIr::Limiter => Node::Text(""),
            SegmentIr::Braces(_b) => todo!(),
            SegmentIr::Math(_) => todo!(),
            SegmentIr::Link(_) => todo!(),
            SegmentIr::Image(_) => todo!(),
            SegmentIr::InlineMacro(_) => todo!(),
            SegmentIr::InlineHtml(h) => h.into(),
            SegmentIr::Format(_f) => todo!(),
            SegmentIr::Code(_c) => todo!(),
        }
    }
}

fn convert_segments(segments: Vec<SegmentIr<'_>>) -> Vec<Node<'_>> {
    segments.into_iter().map(From::from).collect()
}
