use crate::indent::Indents;
use crate::items::{Node, ParentKind};
use crate::marker::ParseLineStart;
use crate::{Input, Parse};

/// A heading, e.g.
///
/// ```md
/// ## Level-2 heading
/// ```
#[derive(Debug, Clone)]
pub struct Heading {
    pub level: u8,
    pub content: Vec<Node>,
}

impl Heading {
    pub fn parser(ind: Indents<'_>) -> ParseHeading<'_> {
        ParseHeading { ind }
    }
}

pub struct ParseHeading<'a> {
    pub ind: Indents<'a>,
}

impl Parse for ParseHeading<'_> {
    type Output = Heading;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse(ParseLineStart)?;
        input.parse('#')?;
        let mut level = 1;
        while input.parse('#').is_some() {
            level += 1;
            if level > 6 {
                return None;
            }
        }
        input.parse(' ')?;
        let content =
            input.parse(Node::multi_parser(ParentKind::Heading { level }, self.ind))?;

        input.apply();
        Some(Heading { level, content })
    }
}
