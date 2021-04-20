use std::convert::TryInto;

use crate::blocks::Paragraph;
use crate::utils::{Indents, ParseLineBreak, ParseLineEnd, WhileChar};
use crate::{Context, Input, Parse};

use super::Segment;

#[derive(Debug, Clone, PartialEq)]
pub struct Code {
    pub content: Vec<Segment>,
}

impl Code {
    pub(crate) fn parser(ind: Indents<'_>, pass: bool) -> ParseCode<'_> {
        ParseCode { ind, pass }
    }
}

pub(crate) struct ParseCode<'a> {
    ind: Indents<'a>,
    pass: bool,
}

impl Parse for ParseCode<'_> {
    type Output = Code;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse('`')?;
        let len = (1 + input.parse_i(WhileChar('`')).len()).try_into().ok()?;

        let mut content = if self.pass {
            input.parse(Paragraph::parser(self.ind, Context::Code(len)))?.segments
        } else {
            let mut content = Vec::new();

            loop {
                let i = input.rest().find(find_special)?;
                if i > 0 {
                    content.push(Segment::Text(input.bump(i)));
                }

                match input.peek_char().unwrap() {
                    '`' => {
                        if input.parse(ParseCodeEndDelimiter { len }).is_some() {
                            break;
                        } else {
                            let backticks = input.parse_i(WhileChar('`'));
                            content.push(Segment::Text(backticks));
                        }
                    }
                    '\n' | '\r' => {
                        input.parse(ParseLineBreak(self.ind))?;
                        content.push(Segment::Text2(" "));
                        if input.can_parse(ParseLineEnd) {
                            return None;
                        }
                    }
                    c => unreachable!("{:?} was not expected", c),
                }
            }

            content
        };

        if let Some(s) = content.first_mut() {
            match s {
                Segment::Text(s) => {
                    if s.to_str(input.text()).starts_with(' ') {
                        *s = s.get(1..);
                    }
                }
                Segment::Text2(s) => {
                    if s.starts_with(' ') {
                        *s = &s[1..];
                    }
                }
                _ => {}
            }
        }

        if let Some(s) = content.last_mut() {
            match s {
                Segment::Text(s) => {
                    if s.to_str(input.text()).ends_with(' ') {
                        *s = s.get(..s.len() - 1);
                    }
                }
                Segment::Text2(s) => {
                    if s.ends_with(' ') {
                        *s = &s[..s.len() - 1];
                    }
                }
                _ => {}
            }
        }

        input.apply();
        Some(Code { content })
    }
}

fn find_special(c: char) -> bool {
    matches!(c, '`' | '\\' | '\n' | '\r')
}

struct ParseCodeEndDelimiter {
    len: u8,
}

impl Parse for ParseCodeEndDelimiter {
    type Output = ();

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        let backticks = input.parse_i(WhileChar('`')).len();
        if backticks != self.len as usize {
            return None;
        }

        input.apply();
        Some(())
    }
}
