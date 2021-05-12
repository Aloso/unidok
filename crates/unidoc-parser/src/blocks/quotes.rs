use unidoc_repr::ast::blocks::Quote;

use crate::utils::{ParseQuoteMarker, ParseSpacesU8};
use crate::{Context, Indents, Input, Parse};

use super::ParseBlock;

pub(crate) struct ParseQuote<'a> {
    pub ind: Indents<'a>,
}

impl Parse for ParseQuote<'_> {
    type Output = Quote;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        let ind = self.ind.push_indent(input.parse(ParseSpacesU8)?);

        input.parse(ParseQuoteMarker)?;
        let ind = ind.push_quote();

        let content = input.parse(ParseBlock::new_multi(Context::Global, ind))?;

        input.apply();
        Some(Quote { content })
    }

    fn can_parse(&mut self, input: &mut Input) -> bool {
        input.can_parse(ParseQuoteMarker)
    }
}
