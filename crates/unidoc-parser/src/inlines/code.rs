use crate::utils::{Indents, ParseLineBreak, ParseLineEnd, WhileChar};
use crate::{Input, Parse};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Code {
    content: String,
}

impl Code {
    pub(crate) fn parser(ind: Indents<'_>) -> ParseCode<'_> {
        ParseCode { ind }
    }
}

pub(crate) struct ParseCode<'a> {
    ind: Indents<'a>,
}

// TODO: Only allow delimiter in double backticks, i.e. ``$  $`` but not `$  $`

// TODO: Allow formatting in double backticks, i.e. ``**bold** text``

impl Parse for ParseCode<'_> {
    type Output = Code;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();
        input.parse('`')?;
        let double = input.parse('`').is_some();
        let _ = input.parse('$');

        let mut content = String::new();
        let mut esc = false;

        loop {
            match input.rest().find(find_special) {
                Some(i) => {
                    if i > 0 {
                        if esc {
                            content.push('\\');
                        }
                        input.bump(i);
                        esc = false;
                    }
                    match input.peek_char().unwrap() {
                        '`' => {
                            if esc {
                                let backticks = input.parse(WhileChar('`')).unwrap();
                                content.push_str(backticks.to_str(input.text()));
                                esc = false;
                            } else if input.parse(ParseCodeEndDelimiter { double }).is_some() {
                                break;
                            } else {
                                input.bump(1);
                                content.push('`');
                            }
                        }
                        '$' => {
                            if esc {
                                input.bump(1);
                                content.push('$');
                                esc = false;
                            } else if input.parse(ParseCodeEndDelimiter { double }).is_some() {
                                break;
                            } else {
                                input.bump(1);
                                content.push('$');
                            }
                        }
                        '\\' => {
                            if esc {
                                content.push('\\');
                                esc = false;
                            } else {
                                esc = true;
                            }
                        }
                        '\n' | '\r' => {
                            input.parse(ParseLineBreak(self.ind))?;
                            content.push(' ');
                            if input.can_parse(ParseLineEnd) {
                                return None;
                            }
                        }
                        c => unreachable!("{:?} was not expected", c),
                    }
                }
                None => {
                    return None;
                }
            }
        }

        Some(Code { content })
    }
}

fn find_special(c: char) -> bool {
    matches!(c, '`' | '$' | '\\' | '\n' | '\r')
}

struct ParseCodeEndDelimiter {
    double: bool,
}

impl Parse for ParseCodeEndDelimiter {
    type Output = ();

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        let _ = input.parse('$');
        if self.double {
            input.parse("``")?;
        } else {
            input.parse('`')?;
        }
        if let Some('`') = input.peek_char() {
            return None;
        }

        input.apply();
        Some(())
    }
}
