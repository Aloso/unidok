use crate::inlines::segments::{Segment, Segments};
use crate::parsing_mode::ParsingMode;
use crate::utils::{Indents, ParseLineBreak, ParseLineEnd, ParseSpaces, While};
use crate::{Context, Input, Parse};

/// A heading
///
/// ### Examples
///
/// ````markdown
/// Heading 1
/// =======
///
/// Heading 2
/// -------
/// ### Heading 3
/// ````
#[derive(Debug, Clone, PartialEq)]
pub struct Heading {
    pub level: u8,
    pub kind: HeadingKind,
    pub segments: Vec<Segment>,
}

impl Heading {
    pub(crate) fn parser(ind: Indents<'_>) -> ParseHeading<'_> {
        ParseHeading { ind }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HeadingKind {
    /// A heading with leading number signs
    Atx,
    /// A heading underlined with dashes or equal signs
    Setext,
}

pub(crate) struct ParseHeading<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseHeading<'_> {
    type Output = Heading;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse_i(ParseSpaces);
        let level = input.parse(ParseHashes)?;
        let segments = input
            .parse(Segments::parser(self.ind, Context::Heading, ParsingMode::new_all()))?
            .into_segments_no_underline_zero()?;

        input.apply();
        Some(Heading { level, segments, kind: HeadingKind::Atx })
    }

    fn can_parse(&mut self, input: &mut Input) -> bool {
        let mut input = input.start();
        input.parse_i(ParseSpaces);
        input.can_parse(ParseHashes)
    }
}

struct ParseHashes;

impl Parse for ParseHashes {
    type Output = u8;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        input.parse('#')?;
        let mut level = 1;

        while input.parse('#').is_some() {
            level += 1;
            if level > 6 {
                return None;
            }
        }
        if !input.can_parse(ParseLineEnd) {
            input.parse(' ').or_else(|| input.parse('\t'))?;
        }
        Some(level)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Underline {
    Double,
    Single,
}

impl Underline {
    pub(crate) fn parser(ind: Indents<'_>) -> ParseUnderline<'_> {
        ParseUnderline { ind }
    }

    pub fn level(self) -> u8 {
        match self {
            Underline::Double => 1,
            Underline::Single => 2,
        }
    }
}

pub(crate) struct ParseUnderline<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseUnderline<'_> {
    type Output = Underline;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse_i(ParseSpaces);

        let u = if input.parse("--").is_some() {
            input.parse_i(While('-'));
            Underline::Single
        } else if input.parse("==").is_some() {
            input.parse_i(While('='));
            Underline::Double
        } else {
            return None;
        };

        input.parse_i(ParseSpaces);
        input.parse(ParseLineEnd)?;

        input.try_parse(ParseLineBreak(self.ind));

        input.apply();
        Some(u)
    }
}
