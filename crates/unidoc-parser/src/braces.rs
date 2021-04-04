use crate::indent::Indents;
use crate::{Input, Node, NodeParentKind, Parse, ParseNodes};

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

#[cfg(test)]
mod tests {
    use crate::braces::ParseBraces;
    use crate::statics::{IsStatic, StaticBraces, StaticNode as SN};
    use crate::Input;

    macro_rules! braces {
        ($( $e:expr ),* $(,)?) => {
            StaticBraces { content: &[ $($e),* ] }
        };
    }

    #[test]
    fn test_braces() {
        let str = "{this {is} cool}";
        let mut input = Input::new(str);

        let parsed = input.parse(ParseBraces::default());
        let expected = braces![
            SN::Text("this "),
            SN::Braces(braces![SN::Text("is")]),
            SN::Text(" cool"),
        ];
        assert!(parsed.is(Some(expected), str));
    }
}
