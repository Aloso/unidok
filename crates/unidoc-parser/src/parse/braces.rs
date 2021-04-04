use crate::Input;

use super::indent::Indents;
use super::{Node, NodeParentKind, Parse, ParseNodes};

/// A block surrounded by `{braces}`.
#[derive(Debug, Clone)]
pub struct Braces {
    pub content: Vec<Node>,
}

#[derive(Debug, Default, Clone, Copy)]
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
    use super::statics::{IsStatic, StaticBraces, StaticNode};

    let str = "{this {is} cool}";
    let mut input = Input::new(str);

    let parsed = input.parse(ParseBraces::default());
    let expected = StaticBraces {
        content: &[
            StaticNode::Text("this "),
            StaticNode::Braces(StaticBraces { content: &[StaticNode::Text("is")] }),
            StaticNode::Text(" cool"),
        ],
    };
    assert!(parsed.is(Some(expected), str));
}
