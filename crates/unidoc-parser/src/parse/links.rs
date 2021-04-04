use crate::{Input, Parse, StrSlice};

use super::Node;

pub struct ParseLink;
pub struct Link {
    pub href: StrSlice,
    pub text: Option<Vec<Node>>,
}

impl Parse for ParseLink {
    type Output = Link;

    fn parse(&self, _input: &mut Input) -> Option<Self::Output> {
        todo!()
    }
}
