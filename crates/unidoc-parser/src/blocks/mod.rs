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

use crate::utils::Indents;
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
}

impl Block {
    pub(crate) fn parser(context: Context, ind: Indents<'_>) -> ParseBlock<'_> {
        ParseBlock { context, ind }
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
    Braces,
    BracesFirstLine,
    Table,
    LinkOrImg,
    Code(u8),
    Heading,
    Global,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ParseBlock<'a> {
    context: Context,
    ind: Indents<'a>,
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
        } else if let Some(list) = input.parse(List::parser(ind)) {
            Some(Block::List(list))
        } else if let Some(quote) = input.parse(Quote::parser(ind)) {
            Some(Block::Quote(quote))
        } else if let Some(mac) = input.parse(BlockMacro::parser(ind)) {
            Some(Block::BlockMacro(mac))
        } else {
            let p = input.parse(Paragraph::parser(ind, self.context))?;
            if let Some(u) = p.underline {
                Some(Block::Heading(Heading {
                    level: match u {
                        Underline::Double => 1,
                        Underline::Single => 2,
                    },
                    kind: HeadingKind::Setext,
                    content: p.segments,
                }))
            } else {
                Some(Block::Paragraph(p))
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
        let parser = Block::parser(self.context, self.ind);

        let mut v = Vec::new();
        while let Some(node) = input.parse(parser) {
            v.push(node);
        }
        Some(v)
    }
}