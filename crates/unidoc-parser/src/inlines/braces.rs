use unidoc_repr::ast::segments::Braces;

use super::Segments;
use crate::parsing_mode::ParsingMode;
use crate::{Context, Indents, Input, Parse};

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct ParseBraces<'a> {
    pub ind: Indents<'a>,
}

impl Parse for ParseBraces<'_> {
    type Output = Braces;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse('{')?;
        let segments = input
            .parse(Segments::parser(self.ind, Context::InlineBraces, ParsingMode::new_all()))?
            .into_segments_no_underline_zero()?;
        input.parse('}')?;

        input.apply();
        Some(Braces { segments })
    }
}
