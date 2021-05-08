use crate::utils::{Indents, ParseQuoteMarker, ParseSpacesU8};
use crate::{Block, Context, Input, Parse};

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
#[derive(Debug, Clone, PartialEq)]
pub struct Quote {
    pub content: Vec<Block>,
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

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        let ind = self.ind.push_indent(input.parse(ParseSpacesU8)?);

        input.parse(ParseQuoteMarker)?;
        let ind = ind.push_quote();

        let content = input.parse(Block::multi_parser(Context::Global, ind))?;

        input.apply();
        Some(Quote { content })
    }

    fn can_parse(&mut self, input: &mut Input) -> bool {
        input.can_parse(ParseQuoteMarker)
    }
}
