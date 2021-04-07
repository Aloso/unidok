use crate::indent::{Indentation, Indents, ParseQuoteMarker};
use crate::items::{Node, NodeCtx};
use crate::utils::{ParseLineStart, ParseSpaces};
use crate::{Input, Parse};

#[rustfmt::skip]
/// A quote (in HTML, `<blockquote>`).
///
/// ### Syntax
///
/// Every line is prefixed with `>` followed by a space:
///
/// ```markdown
/// > This is
/// > a quote.
/// ```
///
/// Quotes can be nested within other quotes or in lists:
///
/// ```markdown
/// - > A quote containing
///   > > another quote.
///   > > * and a list.
/// ```
///
/// this renders like this:
///
/// - > A quote containing
///   > > another quote.
///   > > * and a list.
///
/// Quotes are block elements, they can contain anything. They can't appear inline (in the middle of a line) however:
///
/// ```markdown
/// Not > a quote
/// ```
///
/// renders like this:
///
/// Not > a quote
#[derive(Debug, Clone)]
pub struct Quote {
    pub content: Vec<Node>,
}

impl Quote {
    pub fn parser(ind: Indents<'_>) -> ParseQuote<'_> {
        ParseQuote { ind }
    }
}

pub struct ParseQuote<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseQuote<'_> {
    type Output = Quote;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        input.parse(ParseLineStart)?;
        let mut input = input.start();

        let indent = input.parse(ParseSpaces)?;
        input.parse(ParseQuoteMarker)?;
        input.set_line_start(true);

        let ind = self.ind.indent(indent);
        let ind = ind.push(Indentation::QuoteMarker);
        let content = input.parse(Node::multi_parser(NodeCtx::ContainerOrGlobal, ind))?;

        input.apply();
        Some(Quote { content })
    }
}
