use crate::ast::blocks::{Bullet, CellAlignment, Fence, ThematicBreakKind};
use crate::ir::segments::Segment;

use super::html::HtmlNode;
use super::macros::Macro;

#[derive(Debug, Clone, PartialEq)]
pub struct AnnBlock<'a> {
    pub macros: Vec<Macro<'a>>,
    pub block: Block<'a>,
}

/// A block. This can be a container (list or blockquote) or a leaf block (code
/// block, comment, heading, table, thematic break, block macro or paragraph).
#[derive(Debug, Clone, PartialEq)]
pub enum Block<'a> {
    CodeBlock(CodeBlock<'a>),
    Paragraph(Paragraph<'a>),
    Heading(Heading<'a>),
    Table(Table<'a>),
    ThematicBreak(ThematicBreak),
    List(List<'a>),
    Quote(Quote<'a>),
    BlockHtml(HtmlNode<'a>),
    Braces(Vec<AnnBlock<'a>>),
    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CodeBlock<'a> {
    pub info: &'a str,
    pub fence: Fence,
    pub lines: Vec<Block<'a>>,
    pub indent: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Paragraph<'a> {
    pub segments: Vec<Segment<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Heading<'a> {
    pub level: u8,
    pub segments: Vec<Segment<'a>>,
    pub slug: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThematicBreak {
    pub len: usize,
    pub kind: ThematicBreakKind,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Table<'a> {
    pub rows: Vec<TableRow<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableRow<'a> {
    pub is_header_row: bool,
    pub cells: Vec<TableCell<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableCell<'a> {
    pub meta: CellMeta,
    pub segments: Vec<Segment<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CellMeta {
    pub is_header_cell: bool,
    pub alignment: CellAlignment,
    pub vertical_alignment: CellAlignment,
    pub rowspan: u16,
    pub colspan: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct List<'a> {
    pub macros: Vec<Macro<'a>>,
    pub bullet: Bullet,
    pub items: Vec<Vec<AnnBlock<'a>>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Quote<'a> {
    pub content: Vec<AnnBlock<'a>>,
}
