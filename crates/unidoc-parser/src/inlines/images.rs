use super::links::{ParseHref, ParseQuotedText};
use super::segments::{Segment, Segments};
use crate::parsing_mode::ParsingMode;
use crate::utils::Indents;
use crate::{Context, Input, Parse};

/// An image that should be shown in the document.
///
/// ### Syntax
///
/// The syntax is similar to links, except that on the left is the alt text,
/// not the content:
///
/// ```markdown
/// ![Alt text](https://www.example.com/image.jpg "a title")
/// ```
///
/// The alt text can be wrapped in `{braces}` to allow line breaks etc.
///
/// If no text is directly above and below the image, the `.block` CSS class is
/// added to the image.
#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    pub href: String,
    pub alt: Vec<Segment>,
    pub title: Option<String>,
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
        let mut input = input.start();

        input.parse("![")?;
        let alt = input
            .parse(Segments::parser(self.ind, Context::LinkOrImg, ParsingMode::new_all()))?
            .into_segments_no_underline_zero()?;
        input.parse("](")?;
        let href = input.parse(ParseHref)?;
        let title = input.parse(ParseQuotedText);
        input.parse(')')?;

        input.apply();
        Some(Image { href, alt, title })
    }
}
