use crate::indent::Indents;
use crate::items::{Node, ParentKind};
use crate::str::StrSlice;
use crate::Parse;

#[derive(Debug, Clone)]
pub struct Attribute {
    pub is_line_start: bool,
    pub content: StrSlice,
}

impl Attribute {
    pub fn parser(ind: Indents<'_>) -> ParseAttribute<'_> {
        ParseAttribute { ind }
    }
}

pub struct ParseAttribute<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseAttribute<'_> {
    type Output = Attribute;

    fn parse(&self, input: &mut crate::Input) -> Option<Self::Output> {
        let is_line_start = input.is_line_start();
        let mut input = input.start();

        input.parse('[')?;
        let content = {
            let mut input2 = input.start();
            input2.parse(Node::multi_parser(ParentKind::Attribute, self.ind, false))?;
            input2.apply()
        };
        input.parse(']')?;

        input.apply();
        Some(Attribute { is_line_start, content })
    }
}
