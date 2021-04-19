use crate::containers::*;
use crate::leaves::*;
use crate::utils::Indents;
use crate::{Input, Parse};

#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    // Leaves
    CodeBlock(CodeBlock),
    Comment(Comment),
    Paragraph(Paragraph),
    Heading(Heading),
    Table(Table),
    ThematicBreak(ThematicBreak),

    // Containers
    List(List),
    Quote(Quote),
    BlockMacro(BlockMacro),
}

impl Block {
    pub fn parser(context: Context, ind: Indents<'_>) -> ParseBlock<'_> {
        ParseBlock { context, ind }
    }

    pub fn multi_parser(context: Context, ind: Indents<'_>) -> ParseNodes<'_> {
        ParseNodes { context, ind }
    }

    pub fn global_parser<'a>() -> ParseNodes<'a> {
        ParseNodes { context: Context::Global, ind: Indents::new() }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Context {
    Braces,
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
            Some(Block::Paragraph(input.parse(Paragraph::parser(ind, self.context))?))
        }
    }

    fn can_parse(&self, _: &mut Input) -> bool {
        true
    }
}

#[derive(Debug)]
pub struct ParseNodes<'a> {
    context: Context,
    ind: Indents<'a>,
}

impl Parse for ParseNodes<'_> {
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
