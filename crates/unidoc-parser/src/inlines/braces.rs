use crate::utils::Indents;
use crate::{Input, Node, NodeCtx, Parse};

/// A block surrounded by `{braces}`. The braces are not visible in the
/// generated document.
///
/// If the braces have an attribute, the content of the braces is wrapped in a
/// `<div>` or `<span>` element (a `<div>` element is used if the braces contain
/// multiple paragraphs or at least one block-level element). Otherwise, the
/// content of the braces is inserted into the document directly.
///
/// ### Syntax
///
/// Braces can contain multiple lines, even if they appear in a single-line
/// element. For example:
///
/// ````markdown
/// # A heading {
/// - This is a list
/// - within a heading
/// }
///
/// |===
/// | A table cell
/// | Another table cell
/// |{ A large table cell.
///
/// Containing multiple paragraphs. }
/// | ===
/// ````
#[derive(Debug, Clone, PartialEq)]
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
        let content = input.parse(Node::multi_parser(NodeCtx::Braces, self.ind))?;
        input.parse('}')?;

        input.apply();
        Some(Braces { content })
    }
}

#[cfg(test)]
mod tests {
    use crate::containers::Bullet;
    use crate::inlines::Braces;
    use crate::statics::{
        StaticEscaped, StaticList, StaticMath, StaticNode as SN, StaticQuote,
        StaticSegment, StaticTable, StaticTableRow,
    };
    use crate::utils::Indents;

    #[test]
    fn test_braces() {
        parse!(
            "{this {is} cool}",
            Braces::parser(Indents::new()),
            braces![ln![Text("this "), Braces(braces![ln![Text("is")]]), Text(" cool")]]
        );
        parse!(
            r"{\%this %{is\\} cool%}",
            Braces::parser(Indents::new()),
            braces![ln![
                Escaped(StaticEscaped { text: "%" }),
                Text("this "),
                Math(StaticMath { text: r"is\" }),
                Text(" cool"),
                Text("%"),
            ]]
        );
        parse!(
            "{\nHello world!\n}",
            Braces::parser(Indents::new()),
            braces![ln![], ln![Text("Hello world!")], ln![]]
        );
        parse!(
            "{\n> Hello\n> world!\n}",
            Braces::parser(Indents::new()),
            braces![
                ln![],
                SN::Quote(StaticQuote {
                    content: &[ln![Text(" Hello")], ln![Text(" world!")]]
                }),
            ]
        );
        parse!("{- Hello\n- world}", Braces::parser(Indents::new()), None);
        parse!(
            "{) Hello\n) world\n}",
            Braces::parser(Indents::new()),
            braces![
                ln![Text(") Hello")],
                SN::List(StaticList {
                    indent: 2,
                    bullet: Bullet::Paren { start: 1 },
                    content: &[&[ln![Text("world")]]],
                }),
            ]
        );
        parse!(
            "{\n- Hello\n- world\n}",
            Braces::parser(Indents::new()),
            braces![
                ln![],
                SN::List(StaticList {
                    indent: 2,
                    bullet: Bullet::Dash,
                    content: &[&[ln![Text("Hello")]], &[ln![Text("world")]]]
                }),
            ]
        );
        parse!(
            "{\n| This | is \n| great! \n}",
            Braces::parser(Indents::new()),
            braces![
                ln![],
                SN::Table(StaticTable {
                    content: &[
                        StaticTableRow::Content(&[
                            &[StaticSegment::Text(" This ")],
                            &[StaticSegment::Text(" is ")],
                        ]),
                        StaticTableRow::Content(&[&[StaticSegment::Text(" great! ")]])
                    ]
                }),
            ]
        );
    }
}
