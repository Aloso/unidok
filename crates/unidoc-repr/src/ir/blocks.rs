use crate::ast::blocks::{Bullet, CellAlignment, Fence, ThematicBreakKind};
use crate::ir::macros::MacroArgsIr;
use crate::ir::segments::SegmentIr;

use super::html::HtmlNodeIr;

#[derive(Debug, Clone, PartialEq)]
pub struct AnnBlockIr<'a> {
    pub annotations: Vec<AnnotationIr<'a>>,
    pub block: BlockIr<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnnotationIr<'a> {
    pub name: &'a str,
    pub args: Option<MacroArgsIr<'a>>,
}

/// A block. This can be a container (list or blockquote) or a leaf block (code
/// block, comment, heading, table, thematic break, block macro or paragraph).
#[derive(Debug, Clone, PartialEq)]
pub enum BlockIr<'a> {
    CodeBlock(CodeBlockIr<'a>),
    Paragraph(ParagraphIr<'a>),
    Heading(HeadingIr<'a>),
    Table(TableIr<'a>),
    ThematicBreak(ThematicBreakIr),
    List(ListIr<'a>),
    Quote(QuoteIr<'a>),
    BlockHtml(HtmlNodeIr<'a>),
    Braces(Vec<AnnBlockIr<'a>>),
    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CodeBlockIr<'a> {
    pub info: &'a str,
    pub fence: Fence,
    pub lines: Vec<BlockIr<'a>>,
    pub indent: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParagraphIr<'a> {
    pub segments: Vec<SegmentIr<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HeadingIr<'a> {
    pub level: u8,
    pub segments: Vec<SegmentIr<'a>>,
    pub slug: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThematicBreakIr {
    pub len: usize,
    pub kind: ThematicBreakKind,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableIr<'a> {
    pub rows: Vec<TableRowIr<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableRowIr<'a> {
    pub is_header_row: bool,
    pub cells: Vec<TableCellIr<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableCellIr<'a> {
    pub meta: CellMetaIr,
    pub segments: Vec<SegmentIr<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CellMetaIr {
    pub is_header_cell: bool,
    pub alignment: CellAlignment,
    pub vertical_alignment: CellAlignment,
    pub rowspan: u16,
    pub colspan: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListIr<'a> {
    pub bullet: Bullet,
    pub items: Vec<Vec<AnnBlockIr<'a>>>,
    pub is_loose: bool,
    pub list_style: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuoteIr<'a> {
    pub content: Vec<AnnBlockIr<'a>>,
}
