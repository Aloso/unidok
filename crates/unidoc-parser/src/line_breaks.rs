use crate::indent::{Indentation, Indents, ParseQuoteMarker};
use crate::utils::ParseNSpaces;
use crate::{Input, Parse};

/// A line break. This includes indentation of the following line!
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineBreak;

impl LineBreak {
    pub fn parser(ind: Indents<'_>) -> ParseLineBreak<'_> {
        ParseLineBreak { ind }
    }
}

/// Parses a line break, including left padding
#[derive(Debug, Clone, Copy)]
pub struct ParseLineBreak<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseLineBreak<'_> {
    type Output = LineBreak;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse('\n')?;

        if !matches!(input.peek_char(), Some('\n') | None) {
            parse_recursive(self.ind.root, &mut input)?;
        }

        input.set_line_start(true);

        input.apply();
        Some(LineBreak)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(super) enum INode<'a> {
    Node { ind: Indentation, next: &'a INode<'a> },
    Tail,
}

impl Default for INode<'_> {
    fn default() -> Self {
        INode::Tail
    }
}

fn parse_recursive(node: INode<'_>, input: &mut Input) -> Option<()> {
    match node {
        INode::Node { ind, next } => {
            parse_recursive(*next, input)?;
            match ind {
                Indentation::Spaces(s) => input.parse(ParseNSpaces(s.into())),
                Indentation::QuoteMarker => input.parse(ParseQuoteMarker),
            }
        }
        INode::Tail => Some(()),
    }
}
