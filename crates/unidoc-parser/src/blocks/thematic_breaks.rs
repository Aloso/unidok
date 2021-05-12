use unidoc_repr::ast::blocks::{ThematicBreak, ThematicBreakKind};

use crate::utils::{ParseLineEnd, ParseSpaces, While};
use crate::{Indents, Input, Parse};

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct ParseThematicBreak<'a> {
    pub ind: Indents<'a>,
}

impl Parse for ParseThematicBreak<'_> {
    type Output = ThematicBreak;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse_i(ParseSpaces);

        let (kind, parser) = if input.parse("***").is_some() {
            (ThematicBreakKind::Stars, While('*'))
        } else if input.parse("___").is_some() {
            (ThematicBreakKind::Underscores, While('_'))
        } else if input.parse("---").is_some() {
            (ThematicBreakKind::Dashes, While('-'))
        } else {
            return None;
        };
        let len = 3 + input.parse_i(parser).len();

        input.parse_i(ParseSpaces);
        input.parse(ParseLineEnd)?;

        input.apply();
        Some(ThematicBreak { len, kind })
    }
}

#[test]
fn test_hr() {
    use crate::utils::ParseLineBreak;
    use ThematicBreakKind::*;

    let mut input = Input::new("  *******   \n    ---\n**\n___");
    let parser = ParseThematicBreak::default();

    assert_eq!(input.parse(parser), Some(ThematicBreak { len: 7, kind: Stars }));
    input.parse(ParseLineBreak::default()).unwrap();
    assert_eq!(input.parse(parser), Some(ThematicBreak { len: 3, kind: Dashes }));
    assert_eq!(input.parse(parser), None);
    input.bump(3);
    input.parse(ParseLineBreak::default()).unwrap();
    assert_eq!(input.parse(parser), Some(ThematicBreak { len: 3, kind: Underscores }));
}
