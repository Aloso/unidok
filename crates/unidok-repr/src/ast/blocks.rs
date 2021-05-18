use detached_str::StrSlice;

use crate::ast::html::HtmlNodeAst;
use crate::ast::macros::BlockMacro;
use crate::ast::segments::SegmentAst;
use crate::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum BlockAst {
    CodeBlock(CodeBlockAst),
    Paragraph(ParagraphAst),
    Heading(HeadingAst),
    Table(TableAst),
    ThematicBreak(ThematicBreakAst),
    List(ListAst),
    Quote(QuoteAst),
    BlockMacro(BlockMacro),
    BlockHtml(HtmlNodeAst),

    Comment(Comment),
    LinkRefDef(LinkRefDef),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CodeBlockAst {
    pub info: StrSlice,
    pub fence_type: FenceType,
    pub lines: Vec<BlockAst>,
    pub indent: u8,

    pub opening_fence: Span,
    pub closing_fence: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FenceType {
    Backticks(u32),
    Tildes(u32),
}

impl FenceType {
    pub fn can_close(self, opening_fence: FenceType) -> bool {
        match (opening_fence, self) {
            (FenceType::Backticks(a), FenceType::Backticks(b)) => a <= b,
            (FenceType::Tildes(a), FenceType::Tildes(b)) => a <= b,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Comment {
    pub content: StrSlice,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HeadingAst {
    pub level: u8,
    pub kind: HeadingKind,
    pub segments: Vec<SegmentAst>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HeadingKind {
    /// A heading with leading number signs
    Atx,
    /// A heading underlined with dashes or equal signs
    Setext,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParagraphAst {
    pub segments: Vec<SegmentAst>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LinkRefDef {
    pub name: StrSlice,
    pub url: StrSlice,
    pub title: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableAst {
    pub rows: Vec<TableRowAst>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableRowAst {
    pub is_header_row: bool,
    pub cells: Vec<TableCellAst>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableCellAst {
    pub meta: CellMetaAst,
    pub segments: Vec<SegmentAst>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CellMetaAst {
    pub is_header_cell: bool,
    pub alignment: CellAlignment,
    pub vertical_alignment: CellAlignment,
    pub rowspan: u16,
    pub colspan: u16,
}

impl Default for CellMetaAst {
    fn default() -> Self {
        CellMetaAst {
            is_header_cell: false,
            alignment: CellAlignment::Unset,
            vertical_alignment: CellAlignment::Unset,
            rowspan: 1,
            colspan: 1,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CellAlignment {
    Unset,
    LeftTop,
    RightBottom,
    Center,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThematicBreakAst {
    pub len: usize,
    pub kind: ThematicBreakKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThematicBreakKind {
    Dashes,
    Stars,
    Underscores,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListAst {
    pub indent_spaces: u8,
    pub bullet: Bullet,
    pub items: Vec<Vec<BlockAst>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bullet {
    Dash,
    Plus,
    Star,
    Dot { start: u32 },
    Paren { start: u32 },
}

impl Bullet {
    pub fn kind(self) -> ListKind {
        match self {
            Bullet::Dash => ListKind::Dashes,
            Bullet::Plus => ListKind::Pluses,
            Bullet::Star => ListKind::Stars,
            Bullet::Dot { .. } => ListKind::Dots,
            Bullet::Paren { .. } => ListKind::Parens,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListKind {
    Dashes,
    Pluses,
    Stars,
    Dots,
    Parens,
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuoteAst {
    pub content: Vec<BlockAst>,
}
