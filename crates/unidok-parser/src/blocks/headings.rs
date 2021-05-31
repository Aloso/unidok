use aho_corasick::AhoCorasick;
use unidok_repr::ast::blocks::{HeadingAst, HeadingKind};

use crate::inlines::Segments;
use crate::parsing_mode::ParsingMode;
use crate::utils::{ParseLineBreak, ParseLineEnd, ParseSpaces, While};
use crate::{Context, Indents, Input, Parse};

pub(crate) struct ParseHeading<'a> {
    pub ind: Indents<'a>,
    pub ac: &'a AhoCorasick,
}

impl Parse for ParseHeading<'_> {
    type Output = HeadingAst;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse_i(ParseSpaces);
        let level = input.parse(ParseHashes)?;
        let segments = input
            .parse(Segments::parser(self.ind, Context::Heading, ParsingMode::new_all(), self.ac))?
            .into_segments_no_underline_zero()?;

        let heading = HeadingAst { level, segments, kind: HeadingKind::Atx };

        input.apply();
        Some(heading)
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
pub(crate) enum Underline {
    Double,
    Single,
}

impl Underline {
    pub fn parser(ind: Indents<'_>) -> ParseUnderline<'_> {
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
