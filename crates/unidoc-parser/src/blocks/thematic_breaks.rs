use crate::utils::{Indents, ParseLineBreak, ParseSpaces, While};
use crate::{Input, Parse};

/// A thematic break, consisting of at least three stars (`***`) or underscores
/// (`___`).
///
/// This is usually rendered as a horizontal ruler (`<hr>`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThematicBreak {
    pub len: usize,
    pub kind: ThematicBreakKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThematicBreakKind {
    Dashes,
    Stars,
    Underscores,
}

impl ThematicBreak {
    pub(crate) fn parser(ind: Indents<'_>) -> ParseThematicBreak<'_> {
        ParseThematicBreak { ind }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct ParseThematicBreak<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseThematicBreak<'_> {
    type Output = ThematicBreak;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
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
        input.parse(ParseLineBreak(self.ind))?;

        input.apply();
        Some(ThematicBreak { len, kind })
    }
}

#[test]
fn test_hr() {
    use ThematicBreakKind::*;

    let mut input = Input::new("  *******   \n    ---\n**\n___");
    let parser = ParseThematicBreak::default();

    assert_eq!(input.parse(parser), Some(ThematicBreak { len: 7, kind: Stars }));
    assert_eq!(input.parse(parser), Some(ThematicBreak { len: 3, kind: Dashes }));
    assert_eq!(input.parse(parser), None);
    input.bump(3);
    assert_eq!(input.parse(parser), Some(ThematicBreak { len: 3, kind: Underscores }));
}
