use unidok_repr::ast::html::HtmlComment;

use crate::utils::{ParseLineBreak, Until};
use crate::{Indents, Input, Parse};

pub(crate) struct ParseHtmlComment<'a> {
    pub ind: Indents<'a>,
}

impl Parse for ParseHtmlComment<'_> {
    type Output = HtmlComment;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse("<!--")?;
        let mut text = String::new();
        loop {
            let s = input.parse_i(Until(|c| matches!(c, '-' | '\n' | '\r')));
            text.push_str(&input[s]);
            match input.peek_char() {
                Some('-') => {
                    if input.parse("-->").is_some() {
                        break;
                    } else {
                        input.bump(1);
                        text.push('-');
                    }
                }
                None => {
                    break;
                }
                _ => {
                    if input.parse(ParseLineBreak(self.ind)).is_some() {
                        text.push('\n');
                    } else {
                        let s = input.bump(1);
                        text.push_str(&input[s]);
                    }
                }
            }
        }

        input.apply();
        Some(HtmlComment { text })
    }
}
