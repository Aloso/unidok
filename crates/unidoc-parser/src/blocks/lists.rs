use crate::utils::{Indents, ParseLineBreak, ParseLineEnd, ParseNSpaces, ParseSpacesU8, While};
use crate::{Block, Context, Parse};

/// A list
///
/// ### Examples
///
/// ````md
/// - List item 1
/// - List item 2
///
/// + List item 3
/// + List item 4
///
/// 5. List item 5
/// 0. List item 6
/// ````
#[derive(Debug, Clone, PartialEq)]
pub struct List {
    pub indent_spaces: u8,
    pub bullet: Bullet,
    pub items: Vec<Vec<Block>>,
    pub is_loose: bool,
    pub list_style: Option<String>,
}

impl List {
    pub(crate) fn parser<'a>(
        ind: Indents<'a>,
        is_loose: bool,
        list_style: &'a mut Option<String>,
    ) -> ParseList<'a> {
        ParseList { ind, is_loose, list_style }
    }
}

pub(crate) struct ParseList<'a> {
    ind: Indents<'a>,
    is_loose: bool,
    list_style: &'a mut Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bullet {
    Dash,
    Plus,
    Star,
    Dot { start: u32 },
    Paren { start: u32 },
}

impl Bullet {
    pub fn kind(self) -> ListKind {
        match self {
            Bullet::Dash => ListKind::Dashes,
            Bullet::Plus => ListKind::Pluses,
            Bullet::Star => ListKind::Stars,
            Bullet::Dot { .. } => ListKind::Dots,
            Bullet::Paren { .. } => ListKind::Parens,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListKind {
    Dashes,
    Pluses,
    Stars,
    Dots,
    Parens,
}

impl Parse for ParseList<'_> {
    type Output = List;

    fn parse(&mut self, input: &mut crate::Input) -> Option<Self::Output> {
        let mut input = input.start();

        let (mut indent_spaces, bullet) = input.parse(ParseBullet { first: true })?;

        let mut items = Vec::new();
        loop {
            let ind = self.ind.push_indent(indent_spaces);

            let content_parser = Block::multi_parser(Context::Global, ind);
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
        Some(List {
            indent_spaces,
            bullet,
            items,
            is_loose: self.is_loose,
            list_style: self.list_style.take(),
        })
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
