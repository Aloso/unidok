use aho_corasick::AhoCorasick;
use unidok_repr::ast::blocks::QuoteAst;

use crate::parsing_mode::ParsingMode;
use crate::state::ParsingState;
use crate::utils::{ParseQuoteMarker, ParseSpacesU8};
use crate::{Context, Indents, Input, Parse};

use super::ParseBlock;

pub(crate) struct ParseQuote<'a> {
    pub ind: Indents<'a>,
    pub mode: Option<ParsingMode>,
    pub ac: &'a AhoCorasick,
}

impl Parse for ParseQuote<'_> {
    type Output = QuoteAst;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        let ind = self.ind.push_indent(input.parse(ParseSpacesU8)?);

        input.parse(ParseQuoteMarker)?;
        let ind = ind.push_quote();

        let content = input.parse(ParseBlock::new_multi(
            self.mode,
            ParsingState::new(ind, Context::Global, self.ac),
        ))?;

        input.apply();
        Some(QuoteAst { content })
    }

    fn can_parse(&mut self, input: &mut Input) -> bool {
        input.can_parse(ParseQuoteMarker)
    }
}
