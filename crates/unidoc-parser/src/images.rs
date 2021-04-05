use crate::basic::UntilChar;
use crate::indent::Indents;
use crate::items::{Node, ParentKind};
use crate::str::StrSlice;
use crate::{Input, Parse};

/// An image that should be shown in the document.
///
/// ### Syntax
///
/// The syntax is similar to links, except that on the right is the alt text,
/// not the content:
///
/// ```markdown
/// <!https://www.example.com/image.jpg Alt text>
/// ```
///
/// Adding the `[link]` attribute wraps the image in a link, so users can click
/// on the image to open it. Adding the `[link, _blank]` makes the image open in
/// a new tab. This can also be configured globally.
///
/// The alt text can be wrapped in `{braces}` to allow line breaks etc.
///
/// If no text is directly above and below the image, the `.block` CSS class is
/// added to the image.
///
/// #### TODO:
/// - Consider adding support for regular `[]()` CommonMark links for better
///   compatibility.
/// - Consider adding support for auto-links.
#[derive(Debug, Clone)]
pub struct Image {
    pub href: StrSlice,
    pub alt: Option<Vec<Node>>,
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

        input.parse("<!")?;
        let href = input.parse(UntilChar(|c| matches!(c, ' ' | '\n' | '>')))?;
        let alt = if input.parse(' ').is_some() || input.parse('\n').is_some() {
            let parser = Node::multi_parser(ParentKind::LinkOrImg, self.ind, false);
            Some(input.parse(parser)?)
        } else {
            None
        };
        input.parse('>')?;

        input.apply();
        Some(Image { href, alt })
    }
}
