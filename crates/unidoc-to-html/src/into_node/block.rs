use std::mem::take;

use itertools::Itertools;
use unidoc_parser::blocks::{Bullet, CellAlignment};
use unidoc_parser::html::ElemName;
use unidoc_parser::ir::{
    BlockIr, CodeBlockIr, HeadingIr, ListIr, ParagraphIr, QuoteIr, TableCellIr, TableIr,
    ThematicBreakIr,
};

use super::helpers::into_nodes_trimmed;
use crate::{Attr, Element, IntoNode, IntoNodes, Node};

impl<'a> IntoNode<'a> for BlockIr<'a> {
    fn into_node(self) -> Node<'a> {
        match self {
            BlockIr::CodeBlock(c) => c.into_node(),
            BlockIr::Comment(_) => Node::Fragment(vec![]),
            BlockIr::Paragraph(p) => p.into_node(),
            BlockIr::Heading(h) => h.into_node(),
            BlockIr::ThematicBreak(t) => t.into_node(),
            BlockIr::Table(t) => t.into_node(),
            BlockIr::BlockHtml(h) => h.into_node(),
            BlockIr::List(l) => l.into_node(),
            BlockIr::Quote(q) => q.into_node(),
            BlockIr::BlockMacro(_) => todo!(),
        }
    }
}

fn into_nodes_tight(blocks: Vec<BlockIr<'_>>) -> Vec<Node<'_>> {
    let mut result = Vec::new();

    for block in blocks {
        match block {
            BlockIr::CodeBlock(c) => result.push(c.into_node()),
            BlockIr::Comment(_) => result.push(Node::Fragment(vec![])),
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
            BlockIr::BlockMacro(_) => todo!(),
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

        let text = if self.lines.is_empty() {
            String::new()
        } else {
            self.lines.into_iter().join("\n") + "\n"
        };

        let content = Node::Text2(text);
        let code = Node::Element(Element {
            name: ElemName::Code,
            attrs,
            content: Some(vec![content]),
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
        Node::Text(_) | Node::Text2(_) => false,
        Node::Cdata(_) | Node::Comment(_) | Node::Doctype(_) => true,
        Node::Fragment(f) => f.len() == 1 && should_make_block_single(&f[0]),
    }
}

fn should_make_block_multi(node: &Node) -> bool {
    match node {
        &Node::Element(Element { is_block_level, .. }) => is_block_level,
        Node::Fragment(f) => f.len() == 1 && should_make_block_multi(&f[0]),
        Node::Text(_) | Node::Text2(_) | Node::Cdata(_) | Node::Comment(_) => false,
        Node::Doctype(_) => true,
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

        Node::Element(Element {
            name,
            attrs: vec![],
            content: Some(into_nodes_trimmed(self.segments)),
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
        // TODO: Determine whether the list is loose or tight

        let (name, start) = match self.bullet {
            Bullet::Dash | Bullet::Plus | Bullet::Star => (ElemName::Ul, 1),
            Bullet::Dot { start } | Bullet::Paren { start } => (ElemName::Ol, start),
        };
        let attrs = if start == 1 {
            vec![]
        } else {
            vec![Attr { key: "start", value: Some(start.to_string()) }]
        };

        let loose = self.is_loose;

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
