use crate::blocks::Paragraph;
use crate::utils::Indents;
use crate::{Context, Input, Parse};

use super::Segment;

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
    pub segments: Vec<Segment>,
}

impl Braces {
    pub(crate) fn parser(ind: Indents<'_>) -> ParseBraces<'_> {
        ParseBraces { ind }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct ParseBraces<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseBraces<'_> {
    type Output = Braces;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse('{')?;
        let segments = input.parse(Paragraph::parser(self.ind, Context::Braces))?.segments;
        input.parse('}')?;

        input.apply();
        Some(Braces { segments })
    }
}
