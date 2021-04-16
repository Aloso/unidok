use crate::containers::*;
use crate::leaves::*;
use crate::utils::Indents;
use crate::{Input, Parse};

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
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

impl Node {
    pub fn parser(context: Context, ind: Indents<'_>) -> ParseNode<'_> {
        ParseNode { context, ind }
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
    Heading,
    Global,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ParseNode<'a> {
    context: Context,
    ind: Indents<'a>,
}

impl Parse for ParseNode<'_> {
    type Output = Node;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let ind = self.ind;

        if let Some(comment) = input.parse(Comment::parser(ind)) {
            Some(Node::Comment(comment))
        } else if let Some(tb) = input.parse(ThematicBreak::parser(ind)) {
            Some(Node::ThematicBreak(tb))
        } else if let Some(block) = input.parse(CodeBlock::parser(ind)) {
            Some(Node::CodeBlock(block))
        } else if let Some(heading) = input.parse(Heading::parser(ind)) {
            Some(Node::Heading(heading))
        } else if let Some(list) = input.parse(List::parser(ind)) {
            Some(Node::List(list))
        } else if let Some(quote) = input.parse(Quote::parser(ind)) {
            Some(Node::Quote(quote))
        } else if let Some(table) = input.parse(Table::parser(ind)) {
            Some(Node::Table(table))
        } else if let Some(mac) = input.parse(BlockMacro::parser(ind)) {
            Some(Node::BlockMacro(mac))
        } else {
            Some(Node::Paragraph(input.parse(Paragraph::parser(ind, self.context))?))
        }
    }

    fn can_parse(&self, input: &mut Input) -> bool {
        let ind = self.ind;

        input.can_parse(Comment::parser(ind))
            || input.can_parse(ThematicBreak::parser(ind))
            || input.can_parse(CodeBlock::parser(ind))
            || input.can_parse(Heading::parser(ind))
            || input.can_parse(List::parser(ind))
            || input.can_parse(Quote::parser(ind))
            || input.can_parse(Table::parser(ind))
            || input.can_parse(BlockMacro::parser(ind))
            || input.can_parse(Paragraph::parser(ind, self.context))
    }
}

#[derive(Debug)]
pub struct ParseNodes<'a> {
    context: Context,
    ind: Indents<'a>,
}

impl Parse for ParseNodes<'_> {
    type Output = Vec<Node>;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let parser = Node::parser(self.context, self.ind);

        let mut v = Vec::new();
        while let Some(node) = input.parse(parser) {
            v.push(node);
        }
        Some(v)
    }
}
