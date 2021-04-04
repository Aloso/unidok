use crate::{Input, Parse, StrSlice};

use super::Node;

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

pub struct ParseImage;

impl Parse for ParseImage {
    type Output = Image;

    fn parse(&self, _input: &mut Input) -> Option<Self::Output> {
        todo!()
    }
}
