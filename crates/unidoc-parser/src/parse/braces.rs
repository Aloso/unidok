use crate::Input;

use super::indent::Indents;
use super::{Node, NodeParentKind, Parse, ParseNodes};

/// A block surrounded by `{braces}`.
pub struct Braces {
    pub content: Vec<Node>,
}

pub struct ParseBraces<'a> {
    pub ind: Indents<'a>,
}

impl Parse for ParseBraces<'_> {
    type Output = Braces;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse('{')?;
        let content =
            input.parse(ParseNodes { parent: NodeParentKind::Braces, ind: self.ind })?;
        input.parse('}')?;

        input.apply();
        Some(Braces { content })
    }
}

#[test]
fn test_braces() {
    let _input = Input::new("{this {is} cool}");
}
