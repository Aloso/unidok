use crate::utils::{Indents, ParseLineBreak, ParseLineEnd, WhileChar};
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
    pub content: Vec<Vec<Block>>,
}

impl List {
    pub(crate) fn parser(ind: Indents<'_>) -> ParseList<'_> {
        ParseList { ind }
    }
}

pub(crate) struct ParseList<'a> {
    ind: Indents<'a>,
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

    fn parse(&self, input: &mut crate::Input) -> Option<Self::Output> {
        let mut input = input.start();

        let (indent_spaces, bullet) = input.parse(ParseBullet { first: true })?;
        let ind = self.ind.push_indent(indent_spaces);

        let mut content = Vec::new();
        loop {
            let content_parser = Block::multi_parser(Context::Global, ind);
            content.push(input.parse(content_parser)?);

            if input.parse(ParseLineBreak(self.ind)).is_none() {
                break;
            }

            let mut input2 = input.start();
            if let Some((is, b)) = input2.parse(ParseBullet { first: false }) {
                if is == indent_spaces && b.kind() == bullet.kind() {
                    input2.apply();
                    continue;
                }
            }
            break;
        }

        input.apply();
        Some(List { indent_spaces, bullet, content })
    }
}

struct ParseBullet {
    #[allow(unused)]
    first: bool,
}

impl Parse for ParseBullet {
    type Output = (u8, Bullet);

    fn parse(&self, input: &mut crate::Input) -> Option<Self::Output> {
        let mut input = input.start();

        let indent = input.parse_i(WhileChar(' ')).len();
        if indent > 200 {
            return None;
        }
        let indent = indent as u8 + 2;

        let result = match input.peek_char() {
            Some('-') => (indent, Bullet::Dash),
            Some('+') => (indent, Bullet::Plus),
            Some('*') => (indent, Bullet::Star),
            Some('.') => (indent, Bullet::Dot { start: 1 }),
            Some(')') => (indent, Bullet::Paren { start: 1 }),
            _ => return None,
        };
        input.bump(1);

        if input.parse(' ').is_none() && input.parse(ParseLineEnd).is_none() {
            return None;
        }

        input.apply();
        Some(result)
    }
}
