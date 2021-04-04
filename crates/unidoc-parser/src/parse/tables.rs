use crate::Parse;

use super::indent::Indents;

#[derive(Debug, Clone)]
pub struct Table;

pub struct ParseTable<'a> {
    pub ind: Indents<'a>,
}

impl Parse for ParseTable<'_> {
    type Output = Table;

    fn parse(&self, _input: &mut crate::Input) -> Option<Self::Output> {
        todo!()
    }
}
