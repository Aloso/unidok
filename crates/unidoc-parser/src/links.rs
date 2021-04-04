use crate::{Input, Parse, StrSlice};

use crate::Node;

#[derive(Debug, Clone)]
pub struct Link {
    pub href: StrSlice,
    pub text: Option<Vec<Node>>,
}

pub struct ParseLink;

impl Parse for ParseLink {
    type Output = Link;

    fn parse(&self, _input: &mut Input) -> Option<Self::Output> {
        todo!()
    }
}
