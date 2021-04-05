use crate::indent::Indents;
use crate::items::{Node, ParentKind};
use crate::{Input, Parse};

/// A block surrounded by `{braces}`.
#[derive(Debug, Clone)]
pub struct Braces {
    pub content: Vec<Node>,
}

impl Braces {
    pub fn parser(ind: Indents<'_>) -> ParseBraces<'_> {
        ParseBraces { ind }
    }
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
        let content = input.parse(Node::multi_parser(ParentKind::Braces, self.ind))?;
        input.parse('}')?;

        input.apply();
        Some(Braces { content })
    }
}

#[cfg(test)]
mod tests {
    use crate::braces::ParseBraces;
    use crate::statics::{
        IsStatic, StaticBraces, StaticEscape, StaticMath, StaticNode as SN,
    };
    use crate::Input;

    macro_rules! braces {
        ($( $e:expr ),* $(,)?) => {
            StaticBraces { content: &[ $($e),* ] }
        };
    }

    macro_rules! parse {
        ($text:literal, $expected:expr) => {{
            let str = $text;
            let mut input = Input::new(str);
            let parsed = input.parse(ParseBraces::default());
            if !parsed.is(Some($expected), str) {
                eprintln!(
                    "INPUT: {:?}\n\nEXPECTED: {:#?}\n\nGOT: {:#?}\n",
                    str, $expected, parsed
                );
                panic!("assertion failed");
            }
        }};
    }

    #[test]
    fn test_braces() {
        parse!(
            "{this {is} cool}",
            braces![
                SN::Text("this "),
                SN::Braces(braces![SN::Text("is")]),
                SN::Text(" cool"),
            ]
        );
        parse!(
            r"{\%this %{is\\} cool%}",
            braces![
                SN::Escape(StaticEscape { line_start: false }),
                SN::Text("%"),
                SN::Text("this "),
                SN::Math(StaticMath { text: r"is\" }),
                SN::Text(" cool"),
                SN::Text("%"),
            ]
        );
    }
}
