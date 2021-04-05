use crate::basic::UntilChar;
use crate::indent::Indents;
use crate::items::{Node, ParentKind};
use crate::str::StrSlice;
use crate::{Input, Parse};

/// An image that should be shown in the document, e.g.
///
/// ```markdown
/// <!https://www.example.com/image.jpg Alt text>
/// ```
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
