use std::convert::TryInto;

use crate::parsing_mode::ParsingMode;
use crate::utils::{Indents, ParseLineBreak, ParseLineEnd, While};
use crate::{Context, Input, Parse};

use super::segments::Segments;
use super::Segment;

#[derive(Debug, Clone, PartialEq)]
pub struct Code {
    pub segments: Vec<Segment>,
}

impl Code {
    pub(crate) fn parser(ind: Indents<'_>, mode: Option<ParsingMode>) -> ParseCode<'_> {
        ParseCode { ind, mode }
    }
}

pub(crate) struct ParseCode<'a> {
    ind: Indents<'a>,
    mode: Option<ParsingMode>,
}

impl Parse for ParseCode<'_> {
    type Output = Code;

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
                    segments.push(Segment::Text(input.bump(i)));
                }

                match input.peek_char().unwrap() {
                    '`' => {
                        if input.parse(ParseCodeEndDelimiter { len }).is_some() {
                            break;
                        } else {
                            let backticks = input.parse_i(While('`'));
                            segments.push(Segment::Text(backticks));
                        }
                    }
                    '\n' | '\r' => {
                        input.parse(ParseLineBreak(self.ind))?;
                        segments.push(Segment::Text2(" "));
                        if input.can_parse(ParseLineEnd) {
                            return None;
                        }
                    }
                    c => unreachable!("{:?} was not expected", c),
                }
            }

            segments
        } else {
            let parser = Segments::parser(self.ind, Context::Code(len), mode);
            input.parse(parser)?.into_segments_no_underline()?
        };

        if let Some(s) = segments.first_mut() {
            s.strip_space_start(&input);
        }
        if let Some(s) = segments.last_mut() {
            s.strip_space_end(&input);
        }

        input.apply();
        Some(Code { segments })
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
