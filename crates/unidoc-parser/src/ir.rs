use crate::containers::*;
use crate::inlines::*;
use crate::leaves::*;
use crate::str::StrSlice;
use crate::Block;

/// A document, consisting of multiple [`BlockIr`]s.
#[derive(Debug, Clone, PartialEq)]
pub struct DocIr<'a> {
    pub blocks: Vec<BlockIr<'a>>,
}

/// A block. This can be a container (list or blockquote) or a leaf block (code
/// block, comment, heading, table, thematic break, block macro or paragraph).
#[derive(Debug, Clone, PartialEq)]
pub enum BlockIr<'a> {
    CodeBlock(CodeBlockIr<'a>),
    Comment(CommentIr<'a>),
    Paragraph(ParagraphIr<'a>),
    Heading(HeadingIr<'a>),
    Table(TableIr<'a>),
    ThematicBreak(ThematicBreakIr),
    List(ListIr<'a>),
    Quote(QuoteIr<'a>),
    BlockMacro(BlockMacroIr<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CodeBlockIr<'a> {
    pub info: &'a str,
    pub fence: Fence,
    pub lines: Vec<&'a str>,
    pub indent: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CommentIr<'a> {
    pub content: &'a str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParagraphIr<'a> {
    pub segments: Vec<SegmentIr<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SegmentIr<'a> {
    LineBreak,
    Text(&'a str),
    EscapedText(&'a str),
    Limiter,
    Braces(BracesIr<'a>),
    Math(MathIr),
    Link(LinkIr<'a>),
    Image(ImageIr<'a>),
    Macro(MacroIr<'a>),
    Format(InlineFormatIr<'a>),
    Code(CodeIr<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct HeadingIr<'a> {
    pub level: u8,
    pub content: Vec<SegmentIr<'a>>,
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
    pub contents: Vec<TableCellIr<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableCellIr<'a> {
    pub meta: CellMetaIr<'a>,
    pub segments: Vec<SegmentIr<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CellMetaIr<'a> {
    pub is_header_cell: bool,
    pub alignment: CellAlignment,
    pub vertical_alignment: CellAlignment,
    pub rowspan: u16,
    pub colspan: u16,
    pub bius: Bius,
    pub css: Vec<&'a str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListIr<'a> {
    pub bullet: Bullet,
    pub items: Vec<DocIr<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuoteIr<'a> {
    pub content: DocIr<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockMacroIr<'a> {
    pub name: &'a str,
    pub args: Option<MacroArgs<'a>>,
    pub content: Box<BlockIr<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MacroArgs<'a> {
    args: Vec<MacroArg<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MacroArg<'a> {
    Atom(&'a str),
    KeyValue(&'a str, &'a str),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BracesIr<'a> {
    pub first_line: Option<Vec<SegmentIr<'a>>>,
    pub content: Vec<BlockIr<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MathIr {
    pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LinkIr<'a> {
    pub href: String,
    pub text: Vec<SegmentIr<'a>>,
    pub title: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImageIr<'a> {
    pub href: String,
    pub alt: Vec<SegmentIr<'a>>,
    pub title: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MacroIr<'a> {
    pub name: &'a str,
    pub args: Option<MacroArgs<'a>>,
    pub content: Box<SegmentIr<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InlineFormatIr<'a> {
    pub formatting: Formatting,
    pub content: Vec<SegmentIr<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CodeIr<'a> {
    content: Vec<SegmentIr<'a>>,
}

pub trait IntoIR<'a> {
    type IR: 'a;

    fn into_ir(self, text: &'a str) -> Self::IR;
}

impl<'a> IntoIR<'a> for StrSlice {
    type IR = &'a str;

    fn into_ir(self, text: &'a str) -> Self::IR {
        self.to_str(text)
    }
}

impl<'a> IntoIR<'a> for () {
    type IR = ();

    fn into_ir(self, _: &'a str) -> Self::IR {}
}

impl<'a, T: IntoIR<'a>> IntoIR<'a> for Vec<T> {
    type IR = Vec<T::IR>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        self.into_iter().map(|t| t.into_ir(text)).collect()
    }
}

impl<'a, T: IntoIR<'a>> IntoIR<'a> for Box<T> {
    type IR = Box<T::IR>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        Box::new((*self).into_ir(text))
    }
}

impl<'a, T: IntoIR<'a>> IntoIR<'a> for Option<T> {
    type IR = Option<T::IR>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        self.map(|t| t.into_ir(text))
    }
}

impl<'a> IntoIR<'a> for Block {
    type IR = BlockIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        match self {
            Block::CodeBlock(b) => BlockIr::CodeBlock(b.into_ir(text)),
            Block::Comment(b) => BlockIr::Comment(b.into_ir(text)),
            Block::Paragraph(b) => BlockIr::Paragraph(b.into_ir(text)),
            Block::Heading(b) => BlockIr::Heading(b.into_ir(text)),
            Block::Table(b) => BlockIr::Table(b.into_ir(text)),
            Block::ThematicBreak(b) => BlockIr::ThematicBreak(b.into_ir(text)),
            Block::List(b) => BlockIr::List(b.into_ir(text)),
            Block::Quote(b) => BlockIr::Quote(b.into_ir(text)),
            Block::BlockMacro(b) => BlockIr::BlockMacro(b.into_ir(text)),
        }
    }
}

impl<'a> IntoIR<'a> for CodeBlock {
    type IR = CodeBlockIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        CodeBlockIr {
            info: self.info.into_ir(text),
            fence: self.fence,
            lines: self.lines.into_ir(text),
            indent: self.indent,
        }
    }
}

impl<'a> IntoIR<'a> for Comment {
    type IR = CommentIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        CommentIr { content: self.content.into_ir(text) }
    }
}

impl<'a> IntoIR<'a> for Paragraph {
    type IR = ParagraphIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        ParagraphIr { segments: self.segments.into_ir(text) }
    }
}

impl<'a> IntoIR<'a> for Segment {
    type IR = SegmentIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        match self {
            Segment::LineBreak(_) => SegmentIr::LineBreak,
            Segment::Text(t) => SegmentIr::Text(t.into_ir(text)),
            Segment::Text2(t) => SegmentIr::Text(t),
            Segment::Escaped(esc) => SegmentIr::EscapedText(esc.text.into_ir(text)),
            Segment::Limiter(_) => SegmentIr::Limiter,
            Segment::Braces(b) => SegmentIr::Braces(b.into_ir(text)),
            Segment::Math(b) => SegmentIr::Math(b.into_ir(text)),
            Segment::Link(b) => SegmentIr::Link(b.into_ir(text)),
            Segment::Image(b) => SegmentIr::Image(b.into_ir(text)),
            Segment::Macro(b) => SegmentIr::Macro(b.into_ir(text)),
            Segment::Format(b) => SegmentIr::Format(b.into_ir(text)),
            Segment::Code(b) => SegmentIr::Code(b.into_ir(text)),
        }
    }
}

impl<'a> IntoIR<'a> for Heading {
    type IR = HeadingIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        HeadingIr { level: self.level, content: self.content.into_ir(text) }
    }
}

impl<'a> IntoIR<'a> for ThematicBreak {
    type IR = ThematicBreakIr;

    fn into_ir(self, _: &str) -> Self::IR {
        ThematicBreakIr { len: self.len, kind: self.kind }
    }
}

impl<'a> IntoIR<'a> for Table {
    type IR = TableIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        TableIr { rows: self.rows.into_ir(text) }
    }
}

impl<'a> IntoIR<'a> for TableRow {
    type IR = TableRowIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        TableRowIr { is_header_row: self.is_header_row, contents: self.contents.into_ir(text) }
    }
}

impl<'a> IntoIR<'a> for TableCell {
    type IR = TableCellIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        TableCellIr { meta: self.meta.into_ir(text), segments: self.segments.into_ir(text) }
    }
}

impl<'a> IntoIR<'a> for CellMeta {
    type IR = CellMetaIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        CellMetaIr {
            is_header_cell: self.is_header_cell,
            alignment: self.alignment,
            vertical_alignment: self.vertical_alignment,
            rowspan: self.rowspan,
            colspan: self.colspan,
            bius: self.bius,
            css: self.css.into_ir(text),
        }
    }
}

impl<'a> IntoIR<'a> for List {
    type IR = ListIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        ListIr {
            bullet: self.bullet,
            items: self.content.into_iter().map(|b| DocIr { blocks: b.into_ir(text) }).collect(),
        }
    }
}

impl<'a> IntoIR<'a> for Quote {
    type IR = QuoteIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        QuoteIr { content: DocIr { blocks: self.content.into_ir(text) } }
    }
}

impl<'a> IntoIR<'a> for BlockMacro {
    type IR = BlockMacroIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        BlockMacroIr {
            name: self.name.into_ir(text),
            args: self.args.map(|_| todo!()),
            content: self.content.into_ir(text),
        }
    }
}

impl<'a> IntoIR<'a> for Braces {
    type IR = BracesIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        BracesIr { first_line: self.first_line.into_ir(text), content: self.content.into_ir(text) }
    }
}

impl<'a> IntoIR<'a> for Math {
    type IR = MathIr;

    fn into_ir(self, _: &str) -> Self::IR {
        MathIr { text: self.text }
    }
}

impl<'a> IntoIR<'a> for Link {
    type IR = LinkIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        LinkIr { href: self.href, text: self.text.into_ir(text), title: self.title }
    }
}

impl<'a> IntoIR<'a> for Image {
    type IR = ImageIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        ImageIr { href: self.href, alt: self.alt.into_ir(text), title: self.title }
    }
}

impl<'a> IntoIR<'a> for Macro {
    type IR = MacroIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        MacroIr {
            name: self.name.into_ir(text),
            args: self.args.map(|_| todo!()),
            content: self.content.into_ir(text),
        }
    }
}

impl<'a> IntoIR<'a> for InlineFormat {
    type IR = InlineFormatIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        InlineFormatIr { formatting: self.formatting, content: self.content.into_ir(text) }
    }
}

impl<'a> IntoIR<'a> for Code {
    type IR = CodeIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        CodeIr { content: self.content.into_ir(text) }
    }
}
