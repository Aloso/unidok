use crate::containers::*;
use crate::items::*;
use crate::leaves::*;
use crate::{Input, Parse};

#[derive(Debug, Clone)]
pub enum Node {
    // Leaves
    CodeBlock(CodeBlock),
    Comment(Comment),
    Line(Line),
    Heading(Heading),
    Table(Table),
    ThematicBreak(ThematicBreak),

    // Containers
    List(List),
    Quote(Quote),
}

impl Node {
    pub fn parser(context: NodeCtx, ind: Indents<'_>) -> ParseNode<'_> {
        ParseNode { context, ind }
    }

    pub fn multi_parser(context: NodeCtx, ind: Indents<'_>) -> ParseNodes<'_> {
        ParseNodes { context, ind }
    }

    pub fn global_parser<'a>() -> ParseNodes<'a> {
        ParseNodes { context: NodeCtx::ContainerOrGlobal, ind: Indents::new() }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NodeCtx {
    Braces,
    ContainerOrGlobal,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ParseNode<'a> {
    context: NodeCtx,
    ind: Indents<'a>,
}

impl Parse for ParseNode<'_> {
    type Output = Node;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let ind = self.ind;
        let context = self.context;

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
        } else {
            Some(Node::Line(input.parse(Line::parser(ind, context))?))
        }
    }
}

#[derive(Debug)]
pub struct ParseNodes<'a> {
    context: NodeCtx,
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
