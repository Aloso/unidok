use crate::blocks::macros::BlockMacroContent;
use crate::blocks::*;
use crate::html::*;
use crate::inlines::*;
use crate::macros::{MacroArgs, TokenTree, TokenTreeAtom};
use crate::parsing_mode::ParsingMode;
use crate::{collapse_text, StrSlice};

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
    BlockHtml(HtmlNodeIr<'a>),
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
    InlineMacro(InlineMacroIr<'a>),
    InlineHtml(HtmlNodeIr<'a>),
    Format(InlineFormatIr<'a>),
    Code(CodeIr<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct HeadingIr<'a> {
    pub level: u8,
    pub segments: Vec<SegmentIr<'a>>,
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
    pub items: Vec<Vec<BlockIr<'a>>>,
    pub is_loose: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuoteIr<'a> {
    pub content: Vec<BlockIr<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockMacroIr<'a> {
    pub name: &'a str,
    pub args: Option<MacroArgsIr<'a>>,
    pub content: BlockMacroContentIr<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockMacroContentIr<'a> {
    Prefixed(Box<BlockIr<'a>>),
    Braces(Vec<BlockIr<'a>>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum MacroArgsIr<'a> {
    Raw(&'a str),
    TokenTrees(Vec<TokenTreeIr<'a>>),
    Attrs(Vec<AttrIr<'a>>),
    CellMeta(Vec<CellMetaIr<'a>>),
    ParsingMode(ParsingMode),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenTreeIr<'a> {
    Atom(TokenTreeAtomIr<'a>),
    KV(&'a str, TokenTreeAtomIr<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenTreeAtomIr<'a> {
    Word(&'a str),
    QuotedWord(String),
    Tuple(Vec<TokenTreeIr<'a>>),
    Braces(BracesIr<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BracesIr<'a> {
    pub segments: Vec<SegmentIr<'a>>,
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
pub struct InlineMacroIr<'a> {
    pub name: &'a str,
    pub args: Option<MacroArgsIr<'a>>,
    pub segment: Box<SegmentIr<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InlineFormatIr<'a> {
    pub formatting: Formatting,
    pub segments: Vec<SegmentIr<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CodeIr<'a> {
    pub segments: Vec<SegmentIr<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HtmlNodeIr<'a> {
    Element(HtmlElemIr<'a>),
    ClosingTag(ElemName),
    CData(&'a str),
    Comment(&'a str),
    Doctype(&'a str),
}

#[derive(Debug, Clone, PartialEq)]
pub struct HtmlElemIr<'a> {
    pub name: ElemName,
    pub attrs: Vec<AttrIr<'a>>,
    pub content: Option<ElemContentIr<'a>>,
    pub close: ElemClose,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttrIr<'a> {
    pub key: &'a str,
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElemContentIr<'a> {
    Blocks(Vec<BlockIr<'a>>),
    Inline(Vec<SegmentIr<'a>>),
    Verbatim(&'a str),
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
            Block::BlockHtml(h) => BlockIr::BlockHtml(h.into_ir(text)),
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
        ParagraphIr { segments: collapse_text(self.segments).into_ir(text) }
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
            Segment::InlineMacro(b) => SegmentIr::InlineMacro(b.into_ir(text)),
            Segment::InlineHtml(h) => SegmentIr::InlineHtml(h.into_ir(text)),
            Segment::Format(b) => SegmentIr::Format(b.into_ir(text)),
            Segment::Code(b) => SegmentIr::Code(b.into_ir(text)),
        }
    }
}

impl<'a> IntoIR<'a> for Heading {
    type IR = HeadingIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        HeadingIr { level: self.level, segments: collapse_text(self.segments).into_ir(text) }
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
        TableRowIr { is_header_row: self.is_header_row, cells: self.cells.into_ir(text) }
    }
}

impl<'a> IntoIR<'a> for TableCell {
    type IR = TableCellIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        TableCellIr {
            meta: self.meta.into_ir(text),
            segments: collapse_text(self.segments).into_ir(text),
        }
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
        ListIr { bullet: self.bullet, items: self.items.into_ir(text), is_loose: false }
    }
}

impl<'a> IntoIR<'a> for Quote {
    type IR = QuoteIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        QuoteIr { content: self.content.into_ir(text) }
    }
}

impl<'a> IntoIR<'a> for BlockMacro {
    type IR = BlockMacroIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        BlockMacroIr {
            name: self.name.into_ir(text),
            args: self.args.into_ir(text),
            content: self.content.into_ir(text),
        }
    }
}

impl<'a> IntoIR<'a> for BlockMacroContent {
    type IR = BlockMacroContentIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        match self {
            BlockMacroContent::Prefixed(p) => BlockMacroContentIr::Prefixed(p.into_ir(text)),
            BlockMacroContent::Braces(b) => BlockMacroContentIr::Braces(b.into_ir(text)),
        }
    }
}

impl<'a> IntoIR<'a> for MacroArgs {
    type IR = MacroArgsIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        match self {
            MacroArgs::Raw(r) => MacroArgsIr::Raw(r.into_ir(text)),
            MacroArgs::TokenTrees(t) => MacroArgsIr::TokenTrees(t.into_ir(text)),
            MacroArgs::Attrs(a) => MacroArgsIr::Attrs(a.into_ir(text)),
            MacroArgs::CellMeta(m) => MacroArgsIr::CellMeta(m.into_ir(text)),
            MacroArgs::ParsingMode(p) => MacroArgsIr::ParsingMode(p),
        }
    }
}

impl<'a> IntoIR<'a> for TokenTree {
    type IR = TokenTreeIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        match self {
            TokenTree::Atom(a) => TokenTreeIr::Atom(a.into_ir(text)),
            TokenTree::KV(k, v) => TokenTreeIr::KV(k.into_ir(text), v.into_ir(text)),
        }
    }
}

impl<'a> IntoIR<'a> for TokenTreeAtom {
    type IR = TokenTreeAtomIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        match self {
            TokenTreeAtom::Word(w) => TokenTreeAtomIr::Word(w.into_ir(text)),
            TokenTreeAtom::QuotedWord(q) => TokenTreeAtomIr::QuotedWord(q),
            TokenTreeAtom::Tuple(t) => TokenTreeAtomIr::Tuple(t.into_ir(text)),
            TokenTreeAtom::Braces(b) => TokenTreeAtomIr::Braces(b.into_ir(text)),
        }
    }
}

impl<'a> IntoIR<'a> for Braces {
    type IR = BracesIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        BracesIr { segments: collapse_text(self.segments).into_ir(text) }
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
        LinkIr { href: self.href, text: collapse_text(self.text).into_ir(text), title: self.title }
    }
}

impl<'a> IntoIR<'a> for Image {
    type IR = ImageIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        ImageIr { href: self.href, alt: collapse_text(self.alt).into_ir(text), title: self.title }
    }
}

impl<'a> IntoIR<'a> for InlineMacro {
    type IR = InlineMacroIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        InlineMacroIr {
            name: self.name.into_ir(text),
            args: self.args.into_ir(text),
            segment: self.segment.into_ir(text),
        }
    }
}

impl<'a> IntoIR<'a> for InlineFormat {
    type IR = InlineFormatIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        InlineFormatIr {
            formatting: self.formatting,
            segments: collapse_text(self.segments).into_ir(text),
        }
    }
}

impl<'a> IntoIR<'a> for Code {
    type IR = CodeIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        CodeIr { segments: collapse_text(self.segments).into_ir(text) }
    }
}

impl<'a> IntoIR<'a> for HtmlNode {
    type IR = HtmlNodeIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        match self {
            HtmlNode::Element(e) => HtmlNodeIr::Element(e.into_ir(text)),
            HtmlNode::ClosingTag(c) => HtmlNodeIr::ClosingTag(c),
            HtmlNode::CData(c) => HtmlNodeIr::CData(c.into_ir(text)),
            HtmlNode::Comment(c) => HtmlNodeIr::Comment(c.into_ir(text)),
            HtmlNode::Doctype(d) => HtmlNodeIr::Doctype(d.into_ir(text)),
        }
    }
}

impl<'a> IntoIR<'a> for Doctype {
    type IR = &'a str;

    fn into_ir(self, text: &'a str) -> Self::IR {
        self.text.into_ir(text)
    }
}

impl<'a> IntoIR<'a> for HtmlComment {
    type IR = &'a str;

    fn into_ir(self, text: &'a str) -> Self::IR {
        self.text.into_ir(text)
    }
}

impl<'a> IntoIR<'a> for CDataSection {
    type IR = &'a str;

    fn into_ir(self, text: &'a str) -> Self::IR {
        self.text.into_ir(text)
    }
}

impl<'a> IntoIR<'a> for HtmlElem {
    type IR = HtmlElemIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        HtmlElemIr {
            name: self.name,
            attrs: self.attrs.into_ir(text),
            content: self.content.into_ir(text),
            close: self.close,
        }
    }
}

impl<'a> IntoIR<'a> for HtmlAttr {
    type IR = AttrIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        AttrIr { key: self.key.into_ir(text), value: self.value }
    }
}

impl<'a> IntoIR<'a> for ElemContent {
    type IR = ElemContentIr<'a>;

    fn into_ir(self, text: &'a str) -> Self::IR {
        match self {
            ElemContent::Blocks(b) => ElemContentIr::Blocks(b.into_ir(text)),
            ElemContent::Inline(i) => ElemContentIr::Inline(collapse_text(i).into_ir(text)),
            ElemContent::Verbatim(v) => ElemContentIr::Verbatim(v.into_ir(text)),
        }
    }
}
