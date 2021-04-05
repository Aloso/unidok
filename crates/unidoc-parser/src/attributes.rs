use crate::indent::Indents;
use crate::Parse;

#[derive(Debug, Clone)]
pub struct Attribute;

impl Attribute {
    pub fn parser(ind: Indents<'_>) -> ParseAttribute<'_> {
        ParseAttribute { ind }
    }
}

pub struct ParseAttribute<'a> {
    pub ind: Indents<'a>,
}

impl Parse for ParseAttribute<'_> {
    type Output = Attribute;

    fn parse(&self, _input: &mut crate::Input) -> Option<Self::Output> {
        None
    }
}
