use unidoc_repr::ast::segments::Image;

use super::links::{ParseLinkTargetReference, ParseLinkTargetUrl};
use super::Segments;
use crate::parsing_mode::ParsingMode;
use crate::{Context, Indents, Input, Parse};

pub(crate) struct ParseImage<'a> {
    pub ind: Indents<'a>,
}

impl Parse for ParseImage<'_> {
    type Output = Image;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        if let Some(img) = input.parse(ParseFullImage { ind: self.ind }) {
            Some(img)
        } else {
            let mut input = input.start();
            input.parse('!')?;
            let target = input.parse(ParseLinkTargetReference)?;
            input.apply();
            Some(Image { alt: None, target })
        }
    }
}

pub(crate) struct ParseFullImage<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseFullImage<'_> {
    type Output = Image;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse("![")?;
        let alt = input
            .parse(Segments::parser(self.ind, Context::LinkOrImg, ParsingMode::new_all()))?
            .into_segments_no_underline_zero()?;
        input.parse(']')?;

        let target =
            input.parse(ParseLinkTargetUrl).or_else(|| input.parse(ParseLinkTargetReference))?;

        input.apply();
        Some(Image { alt: Some(alt), target })
    }
}
