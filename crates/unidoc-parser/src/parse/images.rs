use crate::{Input, Parse, StrSlice};

/// An image that should be shown in the document, e.g.
///
/// ```markdown
/// <!https://www.example.com/image.jpg Alt text>
/// ```
pub struct Image {
    pub href: StrSlice,
    pub alt: Option<Vec<Image>>,
}

pub struct ParseImage;

impl Parse for ParseImage {
    type Output = Image;

    fn parse(&self, _input: &mut Input) -> Option<Self::Output> {
        todo!()
    }
}
