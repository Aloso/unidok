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
    ind: Indents<'a>,
}

impl Parse for ParseBraces<'_> {
    type Output = Braces;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse('{')?;
        let content =
            input.parse(Node::multi_parser(ParentKind::Braces, self.ind, true))?;
        input.parse('}')?;

        input.apply();
        Some(Braces { content })
    }
}

#[cfg(test)]
mod tests {
    use crate::indent::Indents;
    use crate::items::{Braces, LineBreak, ListKind};
    use crate::statics::{
        StaticBraces, StaticEscaped, StaticList, StaticMath, StaticNode as SN,
        StaticQuote, StaticTable,
    };

    #[test]
    fn test_braces() {
        parse!(
            "{this {is} cool}",
            Braces::parser(Indents::new()),
            braces![
                SN::Text("this "),
                SN::Braces(braces![SN::Text("is")]),
                SN::Text(" cool"),
            ]
        );
        parse!(
            r"{\%this %{is\\} cool%}",
            Braces::parser(Indents::new()),
            braces![
                SN::Escape(StaticEscaped { line_start: false, text: "%" }),
                SN::Text("this "),
                SN::Math(StaticMath { text: r"is\" }),
                SN::Text(" cool"),
                SN::Text("%"),
            ]
        );
        parse!(
            "{\nHello world!\n}",
            Braces::parser(Indents::new()),
            braces![
                SN::LineBreak(LineBreak),
                SN::Text("Hello world!"),
                SN::LineBreak(LineBreak),
            ]
        );
        parse!(
            "{\n> Hello\n> world!\n}",
            Braces::parser(Indents::new()),
            braces![
                SN::LineBreak(LineBreak),
                SN::Quote(StaticQuote {
                    content: &[
                        SN::Text("Hello"),
                        SN::LineBreak(LineBreak),
                        SN::Text("world!")
                    ]
                }),
                SN::LineBreak(LineBreak),
            ]
        );
        parse!("{- Hello\n- world}", Braces::parser(Indents::new()), None);
        parse!(
            "{- Hello\n- world\n}",
            Braces::parser(Indents::new()),
            braces![
                SN::Text("- Hello"),
                SN::LineBreak(LineBreak),
                SN::List(StaticList {
                    indent: 2,
                    kind: ListKind::Dashes,
                    content: &[&[SN::Text("world")]],
                }),
                SN::LineBreak(LineBreak),
            ]
        );
        parse!(
            "{\n- Hello\n- world\n}",
            Braces::parser(Indents::new()),
            braces![
                SN::LineBreak(LineBreak),
                SN::List(StaticList {
                    indent: 2,
                    kind: ListKind::Dashes,
                    content: &[&[SN::Text("Hello")], &[SN::Text("world")]]
                }),
                SN::LineBreak(LineBreak),
            ]
        );
        parse!(
            "{\n|===\n| This | is \n| great! \n|===\n}",
            Braces::parser(Indents::new()),
            braces![
                SN::LineBreak(LineBreak),
                SN::Table(StaticTable {
                    eq: 3,
                    content: &[
                        &[&[SN::Text(" This ")], &[SN::Text(" is ")]],
                        &[&[SN::Text(" great! ")]]
                    ]
                }),
                SN::LineBreak(LineBreak),
            ]
        );
    }
}
