use crate::utils::{Indents, ParseLineBreak, ParseLineEnd};
use crate::{Input, Node, NodeCtx, Parse};

use super::{Segment, SegmentCtx};

/// A block surrounded by `{braces}`. This is used by inline macros.
///
/// ### Syntax
///
/// Braces can contain multiple lines, even if they appear in a single-line
/// element. For example:
///
/// ````markdown
/// # A heading @{
/// - This is a list
/// - within a heading
/// }
///
/// |===
/// | A table cell
/// | Another table cell
/// |@{ A large table cell.
///
/// Containing multiple paragraphs. }
/// | ===
/// ````
#[derive(Debug, Clone, PartialEq)]
pub struct Braces {
    pub first_line: Option<Vec<Segment>>,
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

        let first_line = if input.can_parse(ParseLineEnd) {
            input.try_parse(ParseLineBreak(self.ind));
            None
        } else {
            let parser = Segment::multi_parser(SegmentCtx::Braces, self.ind);
            Some(input.parse(parser)?)
        };

        let content = input.parse(Node::multi_parser(NodeCtx::Braces, self.ind))?;

        input.parse('}')?;

        input.apply();
        Some(Braces { first_line, content })
    }
}
