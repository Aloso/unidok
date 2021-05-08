pub(crate) mod code_blocks;
pub(crate) mod comments;
pub(crate) mod headings;
pub(crate) mod lists;
pub(crate) mod macros;
pub(crate) mod paragraphs;
pub(crate) mod quotes;
pub(crate) mod tables;
pub(crate) mod thematic_breaks;

pub use code_blocks::{CodeBlock, Fence};
pub use comments::Comment;
pub use headings::{Heading, HeadingKind, Underline};
pub use lists::{Bullet, List};
pub use macros::BlockMacro;
pub use paragraphs::Paragraph;
pub use quotes::Quote;
pub use tables::{Bius, CellAlignment, CellMeta, Table, TableCell, TableRow};
pub use thematic_breaks::{ThematicBreak, ThematicBreakKind};

use crate::html::{ElemName, HtmlNode};
use crate::inlines::segments::Segments;
use crate::parsing_mode::ParsingMode;
use crate::utils::{Indents, ParseLineBreak};
use crate::{Input, Parse};

/// A block
#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    CodeBlock(CodeBlock),
    Comment(Comment),
    Paragraph(Paragraph),
    Heading(Heading),
    Table(Table),
    ThematicBreak(ThematicBreak),
    List(List),
    Quote(Quote),
    BlockMacro(BlockMacro),
    BlockHtml(HtmlNode),
}

impl Block {
    pub(crate) fn parser(
        context: Context,
        ind: Indents<'_>,
        is_loose: bool,
        list_style: Option<String>,
    ) -> ParseBlock<'_> {
        ParseBlock { context, ind, is_loose, list_style }
    }

    pub(crate) fn multi_parser(context: Context, ind: Indents<'_>) -> ParseBlocks<'_> {
        ParseBlocks { context, ind }
    }

    pub(crate) fn global_parser<'a>() -> ParseBlocks<'a> {
        ParseBlocks { context: Context::Global, ind: Indents::new() }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Context {
    BlockBraces,
    Braces,
    Table,
    LinkOrImg,
    Code(u8),
    Heading,
    Html(ElemName),
    Global,
}

impl Context {
    pub fn can_contain_block_macro(self) -> bool {
        !matches!(self, Context::Braces | Context::LinkOrImg | Context::Code(_))
    }

    pub fn get_parent(self) -> Option<ElemName> {
        match self {
            Context::Html(e) => Some(e),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ParseBlock<'a> {
    context: Context,
    ind: Indents<'a>,
    is_loose: bool,
    list_style: Option<String>,
}

impl Parse for ParseBlock<'_> {
    type Output = Block;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let ind = self.ind;

        if let Some(comment) = input.parse(Comment::parser(ind)) {
            Some(Block::Comment(comment))
        } else if let Some(tb) = input.parse(ThematicBreak::parser(ind)) {
            Some(Block::ThematicBreak(tb))
        } else if let Some(block) = input.parse(CodeBlock::parser(ind)) {
            Some(Block::CodeBlock(block))
        } else if let Some(table) = input.parse(Table::parser(ind)) {
            Some(Block::Table(table))
        } else if let Some(heading) = input.parse(Heading::parser(ind)) {
            Some(Block::Heading(heading))
        } else if let Some(list) =
            input.parse(List::parser(ind, self.is_loose, self.list_style.clone()))
        {
            Some(Block::List(list))
        } else if let Some(quote) = input.parse(Quote::parser(ind)) {
            Some(Block::Quote(quote))
        } else if let Some(mac) = input.parse(BlockMacro::parser(
            self.context,
            ind,
            self.is_loose,
            self.list_style.clone(),
        )) {
            Some(Block::BlockMacro(mac))
        } else {
            let segments =
                input.parse(Segments::parser(ind, self.context, ParsingMode::new_all()))?;
            match segments {
                Segments::Empty => None,
                Segments::Some { segments, underline: None } => {
                    Some(Block::Paragraph(Paragraph { segments }))
                }
                Segments::Some { segments, underline: Some(u) } => Some(Block::Heading(Heading {
                    level: u.level(),
                    kind: HeadingKind::Setext,
                    segments,
                })),
            }
        }
    }

    fn can_parse(&self, _: &mut Input) -> bool {
        true
    }
}

#[derive(Debug)]
pub(crate) struct ParseBlocks<'a> {
    context: Context,
    ind: Indents<'a>,
}

impl Parse for ParseBlocks<'_> {
    type Output = Vec<Block>;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        loop {
            if input.parse(ParseLineBreak(self.ind)).is_none() {
                break;
            }
            if input.is_empty() {
                return Some(vec![]);
            }
        }

        let parser = Block::parser(self.context, self.ind, false, None);

        let mut v = Vec::new();
        while let Some(node) = input.parse(parser.clone()) {
            v.push(node);
        }
        Some(v)
    }
}
