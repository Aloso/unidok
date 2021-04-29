use itertools::Itertools;
use unidoc_parser::blocks::{Bullet, CellAlignment};
use unidoc_parser::html::ElemName;
use unidoc_parser::inlines::Formatting;
use unidoc_parser::ir::*;

use crate::{Attr, Element, Node, ToPlaintext};

fn trim_segments_start(segment: SegmentIr<'_>) -> Option<SegmentIr<'_>> {
    match segment {
        SegmentIr::LineBreak | SegmentIr::Limiter => None,
        SegmentIr::Text(mut t) => {
            t = t.trim_start_matches(|c| matches!(c, ' ' | '\t'));
            Some(SegmentIr::Text(t)).filter(|_| !t.is_empty())
        }
        SegmentIr::EscapedText(mut t) => {
            t = t.trim_start_matches(|c| matches!(c, ' ' | '\t'));
            Some(SegmentIr::EscapedText(t)).filter(|_| !t.is_empty())
        }
        s => Some(s),
    }
}

fn trim_segments_end(seg: &mut SegmentIr) -> bool {
    match seg {
        SegmentIr::LineBreak | SegmentIr::Limiter => true,
        SegmentIr::Text(t) | SegmentIr::EscapedText(t) => {
            *t = t.trim_end_matches(|c| matches!(c, ' ' | '\t'));
            t.is_empty()
        }
        _ => false,
    }
}

fn into_nodes_trimmed(mut segments: Vec<SegmentIr<'_>>) -> Vec<Node<'_>> {
    while let Some(seg) = segments.last_mut() {
        if trim_segments_end(seg) {
            segments.pop();
        } else {
            break;
        }
    }

    let mut result = Vec::with_capacity(segments.len());
    let mut iter = segments.into_iter();

    while let Some(seg) = iter.next() {
        if let Some(seg) = trim_segments_start(seg) {
            result.push(seg.into_node());
            result.extend(iter.map(IntoNode::into_node));
            break;
        }
    }

    result
}

pub trait IntoNode<'a> {
    fn into_node(self) -> Node<'a>;
}

pub trait IntoNodes<'a> {
    fn into_nodes(self) -> Vec<Node<'a>>;
}

impl<'a, T: IntoNode<'a>> IntoNodes<'a> for Vec<T> {
    fn into_nodes(self) -> Vec<Node<'a>> {
        self.into_iter().map(IntoNode::into_node).collect()
    }
}

impl<'a> IntoNode<'a> for HtmlNodeIr<'a> {
    fn into_node(self) -> Node<'a> {
        match self {
            HtmlNodeIr::Element(e) => e.into_node(),
            HtmlNodeIr::ClosingTag(_) => Node::Text(""),
            HtmlNodeIr::CData(c) => Node::Cdata(c),
            HtmlNodeIr::Comment(c) => Node::Comment(c),
            HtmlNodeIr::Doctype(d) => Node::Doctype(d),
        }
    }
}

impl<'a> IntoNode<'a> for HtmlElemIr<'a> {
    fn into_node(self) -> Node<'a> {
        let content = self.content.map(elem_content_ir_into_nodes);
        let contains_blocks =
            content.as_ref().map(|c| c.iter().any(Node::is_block_element)).unwrap_or(false);

        Node::Element(Element {
            name: self.name,
            attrs: self.attrs.into_iter().map(From::from).collect(),
            content,
            is_block_level: self.name.is_block_level(),
            contains_blocks,
        })
    }
}

impl<'a> From<AttrIr<'a>> for Attr<'a> {
    fn from(attr: AttrIr<'a>) -> Self {
        Attr { key: attr.key, value: attr.value.map(ToString::to_string) }
    }
}

fn elem_content_ir_into_nodes(content: ElemContentIr<'_>) -> Vec<Node<'_>> {
    match content {
        ElemContentIr::Blocks(b) => b.into_nodes(),
        ElemContentIr::Inline(i) => i.into_nodes(),
        ElemContentIr::Verbatim(v) => vec![Node::Text(v)],
    }
}

impl<'a> IntoNode<'a> for BlockIr<'a> {
    fn into_node(self) -> Node<'a> {
        match self {
            BlockIr::CodeBlock(c) => {
                let attrs = if c.info.trim_start() != "" {
                    vec![Attr { key: "data-language", value: Some(c.info.to_string()) }]
                } else {
                    vec![]
                };

                let content = Node::Text2(c.lines.into_iter().join("\n"));
                let code = Node::Element(Element {
                    name: ElemName::Code,
                    attrs: vec![],
                    content: Some(vec![content]),
                    is_block_level: false,
                    contains_blocks: false,
                });

                Node::Element(Element {
                    name: ElemName::Pre,
                    attrs,
                    content: Some(vec![code]),
                    is_block_level: true,
                    contains_blocks: true,
                })
            }

            BlockIr::Comment(_) => Node::Text(""),

            BlockIr::Paragraph(p) => {
                let segments = into_nodes_trimmed(p.segments);
                if segments.is_empty() {
                    Node::Text("")
                } else {
                    Node::Element(Element {
                        name: ElemName::P,
                        attrs: vec![],
                        content: Some(segments),
                        is_block_level: true,
                        contains_blocks: false,
                    })
                }
            }

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
                content: Some(into_nodes_trimmed(h.segments)),
                is_block_level: true,
                contains_blocks: false,
            }),

            BlockIr::ThematicBreak(_) => Node::Element(Element {
                name: ElemName::Hr,
                attrs: vec![],
                content: None,
                is_block_level: true,
                contains_blocks: false,
            }),

            BlockIr::Table(t) => {
                let rows = t
                    .rows
                    .into_iter()
                    .map(|row| {
                        let is_header_row = row.is_header_row;
                        let cells = row
                            .cells
                            .into_iter()
                            .map(|cell| create_table_cell(is_header_row, cell))
                            .collect();

                        Node::Element(Element {
                            name: ElemName::Tr,
                            attrs: vec![],
                            content: Some(cells),
                            is_block_level: true,
                            contains_blocks: true,
                        })
                    })
                    .collect();

                Node::Element(Element {
                    name: ElemName::Table,
                    attrs: vec![],
                    content: Some(rows),
                    is_block_level: true,
                    contains_blocks: true,
                })
            }

            BlockIr::BlockHtml(h) => h.into_node(),

            BlockIr::List(l) => {
                // TODO: Determine whether the list is loose or tight

                let (name, start) = match l.bullet {
                    Bullet::Dash | Bullet::Plus | Bullet::Star => (ElemName::Ul, 1),
                    Bullet::Dot { start } | Bullet::Paren { start } => (ElemName::Ol, start),
                };
                let attrs = if start == 1 {
                    vec![]
                } else {
                    vec![Attr { key: "start", value: Some(start.to_string()) }]
                };

                let items = l
                    .items
                    .into_iter()
                    .map(|it| {
                        let content = Some(it.blocks.into_nodes());
                        Node::Element(Element {
                            name: ElemName::Li,
                            attrs: vec![],
                            content,
                            is_block_level: true,
                            // TODO: Depends on whether the list is loose or tight!
                            contains_blocks: false,
                        })
                    })
                    .collect();

                Node::Element(Element {
                    name,
                    attrs,
                    content: Some(items),
                    is_block_level: true,
                    contains_blocks: true,
                })
            }

            BlockIr::Quote(q) => {
                let content = Some(q.content.blocks.into_nodes());
                Node::Element(Element {
                    name: ElemName::Blockquote,
                    attrs: vec![],
                    content,
                    is_block_level: true,
                    contains_blocks: true,
                })
            }

            BlockIr::BlockMacro(_) => todo!(),
        }
    }
}

fn create_table_cell(is_header_row: bool, cell: TableCellIr<'_>) -> Node<'_> {
    let name = if is_header_row || cell.meta.is_header_cell { ElemName::Th } else { ElemName::Td };

    let mut attrs = vec![];
    macro_rules! attr {
        ($attrs:ident: $key:literal = $value:expr) => {
            $attrs.push(Attr { key: $key, value: Some($value) });
        };
    }
    if cell.meta.colspan != 1 {
        attr!(attrs: "colspan" = cell.meta.colspan.to_string());
    }
    if cell.meta.rowspan != 1 {
        attr!(attrs: "rowspan" = cell.meta.rowspan.to_string());
    }
    match cell.meta.alignment {
        CellAlignment::Unset => {}
        CellAlignment::LeftTop => {
            attr!(attrs: "align" = "left".to_string());
        }
        CellAlignment::RightBottom => {
            attr!(attrs: "align" = "right".to_string());
        }
        CellAlignment::Center => {
            attr!(attrs: "align" = "center".to_string());
        }
    }
    match cell.meta.vertical_alignment {
        CellAlignment::Unset => {}
        CellAlignment::LeftTop => {
            attr!(attrs: "align" = "top".to_string());
        }
        CellAlignment::RightBottom => {
            attr!(attrs: "align" = "bottom".to_string());
        }
        CellAlignment::Center => {
            attr!(attrs: "align" = "middle".to_string());
        }
    }

    let bius = cell.meta.bius;
    if !bius.is_initial() || !cell.meta.css.is_empty() {
        let mut styles = String::new();
        if bius.is_bold() {
            styles.push_str("font-weight:bold;");
        }
        if bius.is_italic() {
            styles.push_str("font-style:italic;");
        }
        match (bius.is_underline(), bius.is_strikethrough()) {
            (false, false) => {}
            (true, false) => styles.push_str("text-decoration:underline;"),
            (false, true) => styles.push_str("text-decoration:line-through;"),
            (true, true) => styles.push_str("text-decoration:underline line-through;"),
        }
        for style in cell.meta.css {
            styles.push_str(style);
            styles.push(';');
        }
        attr!(attrs: "style" = styles);
    }

    Node::Element(Element {
        name,
        attrs,
        content: Some(into_nodes_trimmed(cell.segments)),
        is_block_level: true,
        contains_blocks: false, // TODO: Depends
    })
}

impl<'a> IntoNode<'a> for SegmentIr<'a> {
    fn into_node(self) -> Node<'a> {
        match self {
            SegmentIr::Text(t) => Node::Text(t),
            SegmentIr::EscapedText(t) => Node::Text(t),
            SegmentIr::LineBreak => Node::Text("\n"),
            SegmentIr::Limiter => Node::Text(""),
            SegmentIr::Braces(b) => Node::Element(Element {
                name: ElemName::Span,
                attrs: vec![],
                content: Some((b.segments).into_nodes()),
                is_block_level: false,
                contains_blocks: false,
            }),
            SegmentIr::Link(l) => l.into_node(),
            SegmentIr::Image(i) => i.into_node(),
            SegmentIr::InlineHtml(h) => h.into_node(),
            SegmentIr::Format(f) => f.into_node(),
            SegmentIr::Code(c) => Node::Element(Element {
                name: ElemName::Code,
                attrs: vec![],
                content: Some(c.segments.into_nodes()),
                is_block_level: false,
                contains_blocks: false,
            }),
            SegmentIr::InlineMacro(_) => todo!(),
            SegmentIr::Math(_) => todo!(),
        }
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
