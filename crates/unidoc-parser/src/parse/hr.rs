use crate::{Input, Parse};

use super::indent::Indents;
use super::marker::{ParseLineEnd, ParseLineStart};

/// A horizontal line, consisting of at least three dashes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HorizontalLine {
    pub len: usize,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ParseHorizontalLine<'a> {
    pub ind: Indents<'a>,
}

impl Parse for ParseHorizontalLine<'_> {
    type Output = HorizontalLine;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse(ParseLineStart)?;
        input.parse("---")?;
        let mut len = 3;
        while input.parse('-').is_some() {
            len += 1;
        }
        while input.parse(' ').is_some() {}
        input.parse(ParseLineEnd)?;

        input.apply();
        Some(HorizontalLine { len })
    }
}

#[test]
fn test_hr() {
    use super::indent::LineBreak;

    let mut input = Input::new("-------\n---\n--\n---");
    let parse_hr = ParseHorizontalLine::default();
    let parse_br = LineBreak(Default::default());

    assert_eq!(input.parse(parse_hr), Some(HorizontalLine { len: 7 }));
    input.parse(parse_br).unwrap();

    assert_eq!(input.parse(parse_hr), Some(HorizontalLine { len: 3 }));
    input.parse(parse_br).unwrap();

    assert_eq!(input.parse(parse_hr), None);
    input.bump(2);
    input.parse(parse_br).unwrap();

    assert_eq!(input.parse(parse_hr), Some(HorizontalLine { len: 3 }));
}
