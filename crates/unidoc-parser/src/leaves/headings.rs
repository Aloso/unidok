use crate::inlines::{Segment, SegmentCtx};
use crate::utils::Indents;
use crate::{Input, Parse};

/// A heading.
///
/// A heading can have one of 6 sizes (in HTML: `<h1>` to `<h6>`). The first
/// heading is the level-1 heading. All level-2 headings after that are
/// subordinate to this, and the level-3 headings are subordinate to the level-2
/// headings, and so on.
///
/// ### Syntax
///
/// ```markdown
/// ## Level-2 heading
///
/// Section
/// ```
///
/// A heading must appear at the beginning of a line. It must start with 1 to 6
/// number signs, followed by at least one space.
///
/// Headings can't contain line breaks, but if a heading contains braces, these
/// braces can contain line breaks.
///
/// Attributes applied to a heading actually applies to the whole section of the
/// heading. For example, this:
///
/// ````markdown
/// [.foo]
/// # Heading
/// bla bla bla
/// ````
///
/// generates HTML similar to this:
///
/// ````html
/// <div class="foo">
///     <h1>Heading</h1>
///     <p>bla bla bla</p>
/// </div>
/// ````
///
/// #### TODO:
/// Implement a way to add an attribute only to the heading, not the whole
/// section
#[derive(Debug, Clone, PartialEq)]
pub struct Heading {
    pub level: u8,
    pub content: Vec<Segment>,
}

impl Heading {
    pub(crate) fn parser(ind: Indents<'_>) -> ParseHeading<'_> {
        ParseHeading { ind }
    }
}

pub(crate) struct ParseHeading<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseHeading<'_> {
    type Output = Heading;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        let level = input.parse(ParseHashes)?;
        let content = input.parse(Segment::multi_parser(SegmentCtx::Other, self.ind))?;

        input.apply();
        Some(Heading { level, content })
    }

    fn can_parse(&self, input: &mut Input) -> bool {
        input.can_parse(ParseHashes)
    }
}

struct ParseHashes;

impl Parse for ParseHashes {
    type Output = u8;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        input.parse('#')?;
        let mut level = 1;

        while input.parse('#').is_some() {
            level += 1;
            if level > 6 {
                return None;
            }
        }
        input.parse(' ')?;
        Some(level)
    }
}
