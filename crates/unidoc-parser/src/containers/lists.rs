use crate::utils::{Indentation, Indents, ParseLineBreak, ParseLineEnd};
use crate::{Node, NodeCtx, Parse, WhileChar};

#[derive(Debug, Clone, PartialEq)]
pub struct List {
    pub indent: u8,
    pub bullet: Bullet,
    pub content: Vec<Vec<Node>>,
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

        let (indent, kind) = input.parse(ParseBullet)?;
        let ind = self.ind.push(Indentation::spaces(indent));

        let mut content = Vec::new();
        loop {
            let content_parser = Node::multi_parser(NodeCtx::ContainerOrGlobal, ind);
            content.push(input.parse(content_parser)?);

            let mut input2 = input.start();
            if input2.parse(ParseLineBreak(self.ind)).is_some() {
                if let Some((indent2, kind2)) = input2.parse(ParseBullet) {
                    if indent2 == indent && kind2 == kind {
                        input2.apply();
                        continue;
                    }
                } else {
                    break;
                }
            }
            break;
        }

        input.apply();
        Some(List { indent, bullet: kind, content })
    }
}

struct ParseBullet;

impl Parse for ParseBullet {
    type Output = (u8, Bullet);

    fn parse(&self, input: &mut crate::Input) -> Option<Self::Output> {
        let mut input = input.start();

        let indent = input.parse(WhileChar(' '))?.len();
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
