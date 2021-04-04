use crate::{Input, Parse};

use crate::indent::{Indentation, Indents, ParseQuoteIndent};
use crate::marker::ParseLineStart;
use crate::{Node, NodeParentKind, ParseNodes};

#[derive(Debug, Clone)]
pub struct Quote {
    pub content: Vec<Node>,
}

pub struct ParseQuote<'a> {
    pub ind: Indents<'a>,
}

impl Parse for ParseQuote<'_> {
    type Output = Quote;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        input.parse(ParseLineStart)?;
        let mut input = input.start();

        input.parse(ParseQuoteIndent)?;
        input.set_line_start(true);

        let ind = self.ind.push(Indentation::Quote);
        let content = input.parse(ParseNodes { parent: NodeParentKind::Quote, ind })?;

        input.apply();
        Some(Quote { content })
    }
}
