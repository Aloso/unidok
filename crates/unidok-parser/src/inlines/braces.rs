use aho_corasick::AhoCorasick;
use unidok_repr::ast::segments::BracesAst;

use super::Segments;
use crate::parsing_mode::ParsingMode;
use crate::{Context, Indents, Input, Parse};

#[derive(Debug, Clone, Copy)]
pub(crate) struct ParseBraces<'a> {
    pub ind: Indents<'a>,
    pub mode: Option<ParsingMode>,
    pub ac: &'a AhoCorasick,
}

impl Parse for ParseBraces<'_> {
    type Output = BracesAst;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse('{')?;
        let segments = input
            .parse(Segments::parser(
                self.ind,
                Context::InlineBraces,
                self.mode.unwrap_or_else(ParsingMode::new_all),
                self.ac,
            ))?
            .into_segments_no_underline_zero()?;
        input.parse('}')?;

        input.apply();
        Some(BracesAst { segments })
    }
}
