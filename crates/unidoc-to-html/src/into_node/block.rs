use std::mem::take;

use unidoc_parser::blocks::{Bullet, CellAlignment};
use unidoc_parser::html::ElemName;
use unidoc_parser::ir::{
    BlockIr, BlockMacroContentIr, BlockMacroIr, CodeBlockIr, HeadingIr, ListIr, MacroArgsIr,
    ParagraphIr, QuoteIr, TableCellIr, TableIr, ThematicBreakIr,
};

use super::helpers::into_nodes_trimmed;
use crate::into_node::segment::add_attributes;
use crate::{Attr, Element, IntoNode, IntoNodes, Node, ToPlaintext};

impl<'a> IntoNode<'a> for BlockIr<'a> {
    fn into_node(self) -> Node<'a> {
        match self {
            BlockIr::CodeBlock(c) => c.into_node(),
            BlockIr::Paragraph(p) => p.into_node(),
            BlockIr::Heading(h) => h.into_node(),
            BlockIr::ThematicBreak(t) => t.into_node(),
            BlockIr::Table(t) => t.into_node(),
            BlockIr::BlockHtml(h) => h.into_node(),
            BlockIr::List(l) => l.into_node(),
            BlockIr::Quote(q) => q.into_node(),
            BlockIr::BlockMacro(m) => m.into_node(),
            BlockIr::Empty => Node::Fragment(vec![]),
        }
    }
}

fn into_nodes_tight(blocks: Vec<BlockIr<'_>>) -> Vec<Node<'_>> {
    let mut result = Vec::new();

    for block in blocks {
        match block {
            BlockIr::CodeBlock(c) => result.push(c.into_node()),
            BlockIr::Paragraph(p) => {
                let segments = into_nodes_trimmed(p.segments);
                if !segments.is_empty() {
                    result.extend(segments);
                    result.push(Node::Element(Element {
                        name: ElemName::Br,
                        attrs: vec![],
                        content: None,
                        is_block_level: false,
                        contains_blocks: false,
                    }))
                }
            }
            BlockIr::Heading(h) => result.push(h.into_node()),
            BlockIr::ThematicBreak(t) => result.push(t.into_node()),
            BlockIr::Table(t) => result.push(t.into_node()),
            BlockIr::BlockHtml(h) => result.push(h.into_node()),
            BlockIr::List(l) => result.push(l.into_node()),
            BlockIr::Quote(q) => result.push(q.into_node()),
            BlockIr::BlockMacro(m) => result.push(m.into_node()),
            BlockIr::Empty => {}
        }
    }
    if let Some(Node::Element(Element { name: ElemName::Br, .. })) = result.last() {
        result.pop();
    }

    result
}

impl<'a> IntoNode<'a> for CodeBlockIr<'a> {
    fn into_node(self) -> Node<'a> {
        let info = self.info.trim_start();
        let attrs = if !info.is_empty() {
            let lang = info.split(|c| matches!(c, ' ' | '\t' | ',' | ';')).next().unwrap();
            vec![Attr { key: "data-language", value: Some(lang.to_string()) }]
        } else {
            vec![]
        };

        let content = if self.lines.is_empty() {
            vec![Node::Text("")]
        } else {
            self.lines
                .into_iter()
                .map(|block| match block {
                    BlockIr::Paragraph(p) => {
                        let mut nodes = p.segments.into_nodes();
                        nodes.push(Node::Text("\n"));
                        Node::Fragment(nodes)
                    }
                    block => block.into_node(),
                })
                .collect()
        };

        let code = Node::Element(Element {
            name: ElemName::Code,
            attrs,
            content: Some(content),
            is_block_level: false,
            contains_blocks: false,
        });

        Node::Element(Element {
            name: ElemName::Pre,
            attrs: vec![],
            content: Some(vec![code]),
            is_block_level: true,
            contains_blocks: false,
        })
    }
}

impl<'a> IntoNode<'a> for ParagraphIr<'a> {
    fn into_node(self) -> Node<'a> {
        let mut segments = into_nodes_trimmed(self.segments);

        if segments.is_empty() {
            Node::Fragment(vec![])
        } else if segments.len() == 1 && should_make_block_single(&segments[0]) {
            segments.pop().unwrap()
        } else if segments.iter().any(|s| should_make_block_multi(s)) {
            let mut fragment = Vec::<Node>::new();
            let mut new_segs = Vec::<Node>::new();

            for s in segments {
                if should_make_block_multi(&s) {
                    if !new_segs.is_empty() {
                        if new_segs.iter().all(Node::is_whitespace) {
                            new_segs.clear();
                        } else {
                            fragment.push(Node::Element(Element {
                                name: ElemName::P,
                                attrs: vec![],
                                content: Some(take(&mut new_segs)),
                                is_block_level: true,
                                contains_blocks: false,
                            }))
                        }
                    }
                    fragment.push(s);
                } else {
                    new_segs.push(s);
                }
            }
            if !new_segs.is_empty() {
                if new_segs.iter().all(Node::is_whitespace) {
                    new_segs.clear();
                } else {
                    fragment.push(Node::Element(Element {
                        name: ElemName::P,
                        attrs: vec![],
                        content: Some(take(&mut new_segs)),
                        is_block_level: true,
                        contains_blocks: false,
                    }))
                }
            }

            Node::Fragment(fragment)
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
}

fn should_make_block_single(node: &Node) -> bool {
    match node {
        &Node::Element(Element { is_block_level, .. }) => is_block_level,
        Node::Text(_) | Node::Text2(_) | Node::Verbatim(_) | Node::Entity(_) => false,
        Node::Cdata(_) | Node::Comment { .. } | Node::Doctype(_) => true,
        Node::Fragment(f) => f.len() == 1 && should_make_block_single(&f[0]),
    }
}

fn should_make_block_multi(node: &Node) -> bool {
    match node {
        &Node::Element(Element { is_block_level, .. }) => is_block_level,
        Node::Fragment(f) => f.len() == 1 && should_make_block_multi(&f[0]),
        Node::Doctype(_) => true,
        _ => false,
    }
}

impl<'a> IntoNode<'a> for HeadingIr<'a> {
    fn into_node(self) -> Node<'a> {
        let name = match self.level {
            1 => ElemName::H1,
            2 => ElemName::H2,
            3 => ElemName::H3,
            4 => ElemName::H4,
            5 => ElemName::H5,
            6 => ElemName::H6,
            l => panic!("Invalid heading level {}", l),
        };
        let mut plain_content = String::new();
        self.to_plaintext(&mut plain_content);
        let plain_content = plain_content;
        let slug = slug::slugify(&plain_content);
        let attrs =
            if slug.is_empty() { vec![] } else { vec![Attr { key: "id", value: Some(slug) }] };

        let content = into_nodes_trimmed(self.segments);

        Node::Element(Element {
            name,
            attrs,
            content: Some(content),
            is_block_level: true,
            contains_blocks: false,
        })
    }
}

impl<'a> IntoNode<'a> for ThematicBreakIr {
    fn into_node(self) -> Node<'a> {
        Node::Element(Element {
            name: ElemName::Hr,
            attrs: vec![],
            content: None,
            is_block_level: true,
            contains_blocks: false,
        })
    }
}

impl<'a> IntoNode<'a> for TableIr<'a> {
    fn into_node(self) -> Node<'a> {
        let rows = self
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
}

impl<'a> IntoNode<'a> for ListIr<'a> {
    fn into_node(self) -> Node<'a> {
        let (name, start) = match self.bullet {
            Bullet::Dash | Bullet::Plus | Bullet::Star => (ElemName::Ul, 1),
            Bullet::Dot { start } | Bullet::Paren { start } => (ElemName::Ol, start),
        };

        let loose = self.is_loose;
        let list_style = &self.list_style;

        let mut attrs =
            if loose { vec![Attr { key: "class", value: Some("loose".into()) }] } else { vec![] };

        if let Some(list_style) = list_style {
            attrs.push(Attr { key: "style", value: Some(format!("list-style: {}", list_style)) });
        }

        if start != 1 {
            attrs.push(Attr { key: "start", value: Some(start.to_string()) })
        };

        let items = self
            .items
            .into_iter()
            .map(|it| {
                let content =
                    if loose { Some(it.into_nodes()) } else { Some(into_nodes_tight(it)) };

                Node::Element(Element {
                    name: ElemName::Li,
                    attrs: vec![],
                    content,
                    is_block_level: true,
                    contains_blocks: loose,
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
}

impl<'a> IntoNode<'a> for QuoteIr<'a> {
    fn into_node(self) -> Node<'a> {
        Node::Element(Element {
            name: ElemName::Blockquote,
            attrs: vec![],
            content: Some(self.content.into_nodes()),
            is_block_level: true,
            contains_blocks: true,
        })
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
            attr!(attrs: "valign" = "top".to_string());
        }
        CellAlignment::RightBottom => {
            attr!(attrs: "valign" = "bottom".to_string());
        }
        CellAlignment::Center => {
            attr!(attrs: "valign" = "middle".to_string());
        }
    }

    Node::Element(Element {
        name,
        attrs,
        content: Some(into_nodes_trimmed(cell.segments)),
        is_block_level: true,
        contains_blocks: false,
    })
}

impl<'a> IntoNode<'a> for BlockMacroIr<'a> {
    fn into_node(self) -> Node<'a> {
        match self.name {
            "" => add_attributes_to_node(self.content.into_node(), self.args),
            _ => self.content.into_node(),
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

impl<'a> IntoNode<'a> for BlockMacroContentIr<'a> {
    fn into_node(self) -> Node<'a> {
        match self {
            BlockMacroContentIr::Prefixed(block) => block.into_node(),
            BlockMacroContentIr::Braces(blocks) => Node::Fragment(blocks.into_nodes()),
        }
    }
}
