use crate::utils::Indents;
use crate::{Input, Parse};

use super::links::{ParseHref, ParseQuotedText};
use super::{Segment, SegmentCtx};

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
    pub fn parser(ind: Indents<'_>) -> ParseImage<'_> {
        ParseImage { ind }
    }
}

pub struct ParseImage<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseImage<'_> {
    type Output = Image;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse("![")?;
        let alt = input.parse(Segment::multi_parser(SegmentCtx::LinkOrImg, self.ind))?;
        input.parse("](")?;
        let href = input.parse(ParseHref)?;
        let title = input.parse(ParseQuotedText);
        input.parse(')')?;

        input.apply();
        Some(Image { href, alt, title })
    }
}
