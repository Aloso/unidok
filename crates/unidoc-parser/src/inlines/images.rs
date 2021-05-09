use super::links::{LinkTarget, ParseLinkTargetReference, ParseLinkTargetUrl};
use super::segments::{Segment, Segments};
use crate::parsing_mode::ParsingMode;
use crate::utils::Indents;
use crate::{Context, Input, Parse};

/// An image that should be shown in the document.
///
/// ### Syntax
///
/// ```markdown
/// ![Alt text](https://www.example.com/image.jpg "a title")
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    pub alt: Option<Vec<Segment>>,
    pub target: LinkTarget,
}

impl Image {
    pub(crate) fn parser(ind: Indents<'_>) -> ParseImage<'_> {
        ParseImage { ind }
    }
}

pub(crate) struct ParseImage<'a> {
    ind: Indents<'a>,
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
