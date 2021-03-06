use std::convert::TryInto;

use aho_corasick::AhoCorasick;
use unidok_repr::ast::segments::{CodeAst, SegmentAst};

use crate::parsing_mode::ParsingMode;
use crate::utils::{ParseLineBreak, ParseLineEnd, While};
use crate::{Context, Indents, Input, Parse};

use super::segments::{strip_space_end, strip_space_start};
use super::Segments;

pub(crate) struct ParseCode<'a> {
    pub ind: Indents<'a>,
    pub mode: Option<ParsingMode>,
    pub ac: &'a AhoCorasick,
}

impl Parse for ParseCode<'_> {
    type Output = CodeAst;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse('`')?;
        let len = (1 + input.parse_i(While('`')).len()).try_into().ok()?;

        let mode = self.mode.unwrap_or_else(ParsingMode::new_nothing);

        let mut segments = if mode.is_nothing() {
            let mut segments = Vec::new();

            loop {
                let i = input.rest().find(find_special)?;
                if i > 0 {
                    segments.push(SegmentAst::Text(input.bump(i)));
                }

                match input.peek_char().unwrap() {
                    '`' => {
                        if input.parse(ParseCodeEndDelimiter { len }).is_some() {
                            break;
                        } else {
                            let backticks = input.parse_i(While('`'));
                            segments.push(SegmentAst::Text(backticks));
                        }
                    }
                    '\n' | '\r' => {
                        input.parse(ParseLineBreak(self.ind))?;
                        segments.push(SegmentAst::Text2(" "));
                        if input.can_parse(ParseLineEnd) {
                            return None;
                        }
                    }
                    c => unreachable!("{:?} was not expected", c),
                }
            }

            segments
        } else {
            let parser = Segments::parser(self.ind, Context::Code(len), mode, self.ac);
            input.parse(parser)?.into_segments_no_underline()?
        };

        if let Some(s) = segments.first_mut() {
            strip_space_start(s, &input);
        }
        if let Some(s) = segments.last_mut() {
            strip_space_end(s, &input);
        }

        input.apply();
        Some(CodeAst { segments })
    }
}

fn find_special(c: char) -> bool {
    matches!(c, '`' | '\n' | '\r')
}

struct ParseCodeEndDelimiter {
    len: u8,
}

impl Parse for ParseCodeEndDelimiter {
    type Output = ();

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        let backticks = input.parse_i(While('`')).len();
        if backticks != self.len as usize {
            return None;
        }

        input.apply();
        Some(())
    }
}
