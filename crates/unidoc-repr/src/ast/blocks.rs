use detached_str::StrSlice;

use crate::ast::html::HtmlNode;
use crate::ast::macros::BlockMacro;
use crate::ast::segments::Segment;

#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    CodeBlock(CodeBlock),
    Paragraph(Paragraph),
    Heading(Heading),
    Table(Table),
    ThematicBreak(ThematicBreak),
    List(List),
    Quote(Quote),
    BlockMacro(BlockMacro),
    BlockHtml(HtmlNode),

    Comment(Comment),
    LinkRefDef(LinkRefDef),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CodeBlock {
    pub info: StrSlice,
    pub fence: Fence,
    pub lines: Vec<Block>,
    pub indent: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fence {
    Backticks(u32),
    Tildes(u32),
}

impl Fence {
    pub fn can_close(self, opening_fence: Fence) -> bool {
        match (opening_fence, self) {
            (Fence::Backticks(a), Fence::Backticks(b)) => a <= b,
            (Fence::Tildes(a), Fence::Tildes(b)) => a <= b,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Comment {
    pub content: StrSlice,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Heading {
    pub level: u8,
    pub kind: HeadingKind,
    pub segments: Vec<Segment>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HeadingKind {
    /// A heading with leading number signs
    Atx,
    /// A heading underlined with dashes or equal signs
    Setext,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Paragraph {
    pub segments: Vec<Segment>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LinkRefDef {
    pub name: StrSlice,
    pub url: StrSlice,
    pub title: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    pub rows: Vec<TableRow>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableRow {
    pub is_header_row: bool,
    pub cells: Vec<TableCell>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableCell {
    pub meta: CellMeta,
    pub segments: Vec<Segment>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CellMeta {
    pub is_header_cell: bool,
    pub alignment: CellAlignment,
    pub vertical_alignment: CellAlignment,
    pub rowspan: u16,
    pub colspan: u16,
}

impl Default for CellMeta {
    fn default() -> Self {
        CellMeta {
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
pub struct ThematicBreak {
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
pub struct List {
    pub indent_spaces: u8,
    pub bullet: Bullet,
    pub items: Vec<Vec<Block>>,
    pub is_loose: bool,
    pub list_style: Option<String>,
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
pub struct Quote {
    pub content: Vec<Block>,
}
