use crate::Parse;

use super::indent::Indents;

#[derive(Debug, Clone)]
pub struct Attribute;

pub struct ParseAttribute<'a> {
    pub ind: Indents<'a>,
}

impl Parse for ParseAttribute<'_> {
    type Output = Attribute;

    fn parse(&self, _input: &mut crate::Input) -> Option<Self::Output> {
        None
    }
}
