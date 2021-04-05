use crate::basic::WhileChar;
use crate::indent::Indents;
use crate::marker::{ParseLineEnd, ParseLineStart};
use crate::{Input, Parse};

/// A horizontal line, consisting of at least three dashes (`---`).
///
/// TODO: Also allow `---` as heading underline, and make sure that horizontal
/// lines are preceded by an empty line. Do something similar for tables?
///
/// The line must be at the beginning of a line and can't contain any other
/// content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HorizontalLine {
    pub len: usize,
}

impl HorizontalLine {
    pub fn parser(ind: Indents<'_>) -> ParseHorizontalLine<'_> {
        ParseHorizontalLine { ind }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ParseHorizontalLine<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseHorizontalLine<'_> {
    type Output = HorizontalLine;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse(ParseLineStart)?;
        input.parse("---")?;
        let len = 3 + input.parse(WhileChar('-'))?.len();
        input.parse(ParseLineEnd)?;

        input.apply();
        Some(HorizontalLine { len })
    }
}

#[test]
fn test_hr() {
    use crate::items::LineBreak;

    let mut input = Input::new("-------\n---\n--\n---");
    let parse_hr = ParseHorizontalLine::default();
    let parse_br = LineBreak::parser(Default::default());

    assert_eq!(input.parse(parse_hr), Some(HorizontalLine { len: 7 }));
    input.parse(parse_br).unwrap();

    assert_eq!(input.parse(parse_hr), Some(HorizontalLine { len: 3 }));
    input.parse(parse_br).unwrap();

    assert_eq!(input.parse(parse_hr), None);
    input.bump(2);
    input.parse(parse_br).unwrap();

    assert_eq!(input.parse(parse_hr), Some(HorizontalLine { len: 3 }));
}
