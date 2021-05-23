use aho_corasick::AhoCorasick;
use unidok_repr::ast::blocks::{Bullet, ListAst};

use crate::parsing_mode::ParsingMode;
use crate::utils::{ParseLineBreak, ParseLineEnd, ParseNSpaces, ParseSpacesU8, While};
use crate::{Context, Indents, Parse};

use super::ParseBlock;

pub(crate) struct ParseList<'a> {
    pub ind: Indents<'a>,
    pub mode: Option<ParsingMode>,
    pub ac: &'a AhoCorasick,
}

impl Parse for ParseList<'_> {
    type Output = ListAst;

    fn parse(&mut self, input: &mut crate::Input) -> Option<Self::Output> {
        let mut input = input.start();

        let (mut indent_spaces, bullet) = input.parse(ParseBullet { first: true })?;

        let mut items = Vec::new();
        loop {
            let ind = self.ind.push_indent(indent_spaces);

            let content_parser = ParseBlock::new_multi(Context::Global, ind, self.mode, self.ac);
            items.push(input.parse(content_parser)?);

            if input.parse(ParseLineBreak(self.ind)).is_none() {
                break;
            }

            let mut input2 = input.start();
            if let Some((is, b)) = input2.parse(ParseBullet { first: false }) {
                if b.kind() == bullet.kind() {
                    indent_spaces = is;
                    input2.apply();
                    continue;
                }
            }
            break;
        }

        input.apply();
        Some(ListAst { indent_spaces, bullet, items })
    }
}

struct ParseBullet {
    #[allow(unused)]
    first: bool,
}

impl Parse for ParseBullet {
    type Output = (u8, Bullet);

    fn parse(&mut self, input: &mut crate::Input) -> Option<Self::Output> {
        let mut input = input.start();

        let indent = input.parse(ParseSpacesU8)?;
        if indent > (u8::MAX - 16) {
            return None;
        }

        let result = match input.peek_char() {
            Some('-') => {
                input.bump(1);
                (indent + 2, Bullet::Dash)
            }
            Some('+') => {
                input.bump(1);
                (indent + 2, Bullet::Plus)
            }
            Some('*') => {
                input.bump(1);
                (indent + 2, Bullet::Star)
            }
            Some('0'..='9') => {
                let num = input.parse_i(While(|c: char| c.is_ascii_digit()));
                if num.len() > 9 {
                    return None;
                }
                let start = num.to_str(input.text()).parse::<u32>().unwrap();

                let bullet = if input.parse('.').is_some() {
                    Bullet::Dot { start }
                } else if input.parse(')').is_some() {
                    Bullet::Paren { start }
                } else {
                    return None;
                };

                (indent + num.len() as u8 + 2, bullet)
            }
            _ => return None,
        };

        if input.parse(ParseNSpaces(1)).is_none() && !input.can_parse(ParseLineEnd) {
            return None;
        }

        input.apply();
        Some(result)
    }
}
