use crate::indent::Indents;
use crate::utils::{ParseLineEnd, ParseLineStart};
use crate::{Input, Parse, WhileChar};

/// A horizontal line, consisting of at least three dashes (`---`), stars
/// (`***`) or underscores (`___`).
///
/// TODO: Also allow `---` as heading underline, and make sure that thematic
/// breaks are preceded by an empty line.
///
/// The line must be at the beginning of a line and can't contain any other
/// content except whitespace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThematicBreak {
    pub len: usize,
}

impl ThematicBreak {
    pub fn parser(ind: Indents<'_>) -> ParseThematicBreak<'_> {
        ParseThematicBreak { ind }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ParseThematicBreak<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseThematicBreak<'_> {
    type Output = ThematicBreak;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse(ParseLineStart)?;

        let parser = if input.parse("---").is_some() {
            WhileChar('-')
        } else if input.parse("***").is_some() {
            WhileChar('*')
        } else if input.parse("___").is_some() {
            WhileChar('_')
        } else {
            return None;
        };
        let len = 3 + input.parse(parser)?.len();

        input.parse(ParseLineEnd)?;

        input.apply();
        Some(ThematicBreak { len })
    }
}

#[test]
fn test_hr() {
    use crate::items::LineBreak;

    let mut input = Input::new("-------\n---\n--\n---");
    let parse_hr = ParseThematicBreak::default();
    let parse_br = LineBreak::parser(Default::default());

    assert_eq!(input.parse(parse_hr), Some(ThematicBreak { len: 7 }));
    input.parse(parse_br).unwrap();

    assert_eq!(input.parse(parse_hr), Some(ThematicBreak { len: 3 }));
    input.parse(parse_br).unwrap();

    assert_eq!(input.parse(parse_hr), None);
    input.bump(2);
    input.parse(parse_br).unwrap();

    assert_eq!(input.parse(parse_hr), Some(ThematicBreak { len: 3 }));
}
