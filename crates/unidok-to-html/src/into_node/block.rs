use std::mem::take;

use unidok_repr::ast::blocks::{Bullet, CellAlignment};
use unidok_repr::ast::html::ElemName;
use unidok_repr::ir::blocks::*;
use unidok_repr::ir::macros::MacroIr;
use unidok_repr::ir::IrState;
use unidok_repr::try_reduce::{Reduced1, TryReduce};

use super::helpers::into_nodes_trimmed;
use super::macros::apply_post_annotations;
use crate::{Attr, Element, IntoNode, IntoNodes, Node};

impl<'a> IntoNode<'a> for AnnBlockIr<'a> {
    fn into_node(self, state: &IrState<'a>) -> Node<'a> {
        let mut node = self.block.into_node(state);
        apply_post_annotations(self.macros, &mut node, state);
        node
    }
}

impl<'a> IntoNode<'a> for BlockIr<'a> {
    fn into_node(self, state: &IrState<'a>) -> Node<'a> {
        match self {
            BlockIr::CodeBlock(c) => c.into_node(state),
            BlockIr::Paragraph(p) => p.into_node(state),
            BlockIr::Heading(h) => h.into_node(state),
            BlockIr::ThematicBreak(t) => t.into_node(state),
            BlockIr::Table(t) => t.into_node(state),
            BlockIr::BlockHtml(h) => h.into_node(state),
            BlockIr::List(l) => l.into_node(state),
            BlockIr::Quote(q) => q.into_node(state),
            BlockIr::Braces(m) => Node::Fragment(m.into_nodes(state)),
            BlockIr::Empty => Node::Fragment(vec![]),
        }
    }
}

fn into_nodes_tight<'a>(blocks: Vec<AnnBlockIr<'a>>, state: &IrState<'a>) -> Vec<Node<'a>> {
    let mut result = Vec::new();

    for block in blocks {
        if let BlockIr::Paragraph(p) = block.block {
            let segments = into_nodes_trimmed(p.segments, state);
            if !segments.is_empty() {
                let mut node = Node::Fragment(segments);
                apply_post_annotations(block.macros, &mut node, state);

                let is_fragment = matches!(node, Node::Fragment(_));
                result.push(node);

                if is_fragment {
                    result.push(Node::Element(elem!(
                        <Br /> is_block_level: false, contains_blocks: false
                    )))
                }
            }
        } else {
            result.push(block.into_node(state))
        }
    }
    if let Some(Node::Element(Element { name: ElemName::Br, .. })) = result.last() {
        result.pop();
    }

    result
}

impl<'a> IntoNode<'a> for CodeBlockIr<'a> {
    fn into_node(self, state: &IrState<'a>) -> Node<'a> {
        let info = self.info.trim_start();
        let attrs = if !info.is_empty() {
            let lang = info.split(|c| matches!(c, ' ' | '\t' | ',' | ';')).next().unwrap();
            vec![Attr { key: "data-language", value: Some(lang.to_string()) }]
        } else {
            vec![]
        };

        let content = if self.lines.is_empty() {
            vec![]
        } else {
            self.lines
                .into_iter()
                .map(|block| match block {
                    BlockIr::Paragraph(p) => {
                        let mut nodes = p.segments.into_nodes(state);
                        nodes.push(Node::Text("\n"));
                        Node::Fragment(nodes)
                    }
                    block => block.into_node(state),
                })
                .collect()
        };

        Node::Element(elem!(<Pre>[
            Node::Element(elem!(
                <Code {attrs}>{content} is_block_level: false, contains_blocks: false
            ))
        ] is_block_level: true, contains_blocks: false))
    }
}

impl<'a> IntoNode<'a> for ParagraphIr<'a> {
    fn into_node(self, state: &IrState<'a>) -> Node<'a> {
        let segments = into_nodes_trimmed(self.segments, state);

        match segments.try_reduce1() {
            Reduced1::Zero => Node::Fragment(vec![]),

            Reduced1::One(node) if matches!(&node, Node::Fragment(f) if f.is_empty()) => {
                Node::Fragment(vec![])
            }

            Reduced1::One(node)
                if should_make_block_single(&node) || should_make_block_multi(&node) =>
            {
                node
            }

            Reduced1::One(node) => Node::Element(elem!(
                <P>[node] is_block_level: true, contains_blocks: false
            )),

            Reduced1::Many(segments) => {
                let mut fragment = Vec::<Node>::new();
                let mut new_segs = Vec::<Node>::new();

                for s in segments {
                    if should_make_block_multi(&s) {
                        if !new_segs.is_empty() {
                            if new_segs.iter().all(Node::is_whitespace) {
                                new_segs.clear();
                            } else {
                                fragment.push(Node::Element(elem!(
                                    <P>{ take(&mut new_segs) } is_block_level: true, contains_blocks: false
                                )))
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
                        fragment.push(Node::Element(elem!(
                            <P>{ take(&mut new_segs) } is_block_level: true, contains_blocks: false
                        )))
                    }
                }

                Node::Fragment(fragment)
            }
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
    fn into_node(self, state: &IrState<'a>) -> Node<'a> {
        let name = match self.level {
            1 => ElemName::H1,
            2 => ElemName::H2,
            3 => ElemName::H3,
            4 => ElemName::H4,
            5 => ElemName::H5,
            6 => ElemName::H6,
            l => panic!("Invalid heading level {}", l),
        };

        let slug = self.slug;
        let attrs =
            if slug.is_empty() { vec![] } else { vec![Attr { key: "id", value: Some(slug) }] };

        let content = into_nodes_trimmed(self.segments, state);

        Node::Element(elem!(
            <{name} {attrs}>{ content } is_block_level: true, contains_blocks: false
        ))
    }
}

impl<'a> IntoNode<'a> for ThematicBreakIr {
    fn into_node(self, _: &IrState) -> Node<'a> {
        Node::Element(elem!(<Hr /> is_block_level: true, contains_blocks: false))
    }
}

impl<'a> IntoNode<'a> for TableIr<'a> {
    fn into_node(self, state: &IrState<'a>) -> Node<'a> {
        let rows = self
            .rows
            .into_iter()
            .map(|row| {
                let is_header_row = row.is_header_row;
                let cells = row
                    .cells
                    .into_iter()
                    .map(|cell| create_table_cell(is_header_row, cell, state))
                    .collect();

                Node::Element(elem!(
                    <Tr>{ cells } is_block_level: true, contains_blocks: true
                ))
            })
            .collect();

        Node::Element(elem!(
            <Table>{ rows } is_block_level: true, contains_blocks: true
        ))
    }
}

impl<'a> IntoNode<'a> for ListIr<'a> {
    fn into_node(self, state: &IrState<'a>) -> Node<'a> {
        let (name, start) = match self.bullet {
            Bullet::Dash | Bullet::Plus | Bullet::Star => (ElemName::Ul, 1),
            Bullet::Dot { start } | Bullet::Paren { start } => (ElemName::Ol, start),
        };

        let mut list_style = None;
        let mut loose = false;

        for r#macro in self.macros {
            match r#macro {
                MacroIr::Loose => loose = true,
                MacroIr::ListStyle(s) => list_style = Some(s),
                r#macro => {
                    panic!("Unexpected macro {:?}", r#macro)
                }
            }
        }

        let mut attrs = if loose { vec![attr!(class = "loose")] } else { vec![] };

        if let Some(list_style) = list_style {
            attrs.push(attr!(style = format!("list-style: {}", list_style)));
        }

        if start != 1 {
            attrs.push(attr!(start = start.to_string()))
        };

        let items = self
            .items
            .into_iter()
            .map(|it| {
                let content =
                    if loose { it.into_nodes(state) } else { into_nodes_tight(it, state) };

                Node::Element(elem!(
                    <Li>{ content } is_block_level: true, contains_blocks: loose
                ))
            })
            .collect();

        Node::Element(elem!(
            <{name} {attrs}>{ items } is_block_level: true, contains_blocks: true
        ))
    }
}

impl<'a> IntoNode<'a> for QuoteIr<'a> {
    fn into_node(self, state: &IrState<'a>) -> Node<'a> {
        Node::Element(elem!(
            <Blockquote>{ self.content.into_nodes(state) } is_block_level: true, contains_blocks: true
        ))
    }
}

fn create_table_cell<'a>(
    is_header_row: bool,
    cell: TableCellIr<'a>,
    state: &IrState<'a>,
) -> Node<'a> {
    let name = if is_header_row || cell.meta.is_header_cell { ElemName::Th } else { ElemName::Td };

    let mut attrs = vec![];

    if cell.meta.colspan != 1 {
        attrs.push(attr!(colspan = cell.meta.colspan.to_string()));
    }
    if cell.meta.rowspan != 1 {
        attrs.push(attr!(rowspan = cell.meta.rowspan.to_string()));
    }
    match cell.meta.alignment {
        CellAlignment::Unset => {}
        CellAlignment::LeftTop => {
            attrs.push(attr!(align = "left"));
        }
        CellAlignment::RightBottom => {
            attrs.push(attr!(align = "right"));
        }
        CellAlignment::Center => {
            attrs.push(attr!(align = "center"));
        }
    }
    match cell.meta.vertical_alignment {
        CellAlignment::Unset => {}
        CellAlignment::LeftTop => {
            attrs.push(attr!(valign = "top"));
        }
        CellAlignment::RightBottom => {
            attrs.push(attr!(valign = "bottom"));
        }
        CellAlignment::Center => {
            attrs.push(attr!(valign = "middle"));
        }
    }

    Node::Element(elem!(
        <{name} {attrs}>{
            into_nodes_trimmed(cell.segments, state)
        } is_block_level: true, contains_blocks: false
    ))
}
