use crate::indent::{Indentation, Indents};
use crate::items::{LineBreak, Node, ParentKind};
use crate::marker::ParseLineStart;
use crate::{Parse, WhileChar};

#[derive(Debug, Clone)]
pub struct List {
    pub indent: u8,
    pub kind: ListKind,
    pub content: Vec<Vec<Node>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListKind {
    Dashes,
    Pluses,
    Stars,
    Dots,
    Commas,
}

impl List {
    pub fn parser(ind: Indents<'_>) -> ParseList<'_> {
        ParseList { ind }
    }
}

pub struct ParseList<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseList<'_> {
    type Output = List;

    fn parse(&self, input: &mut crate::Input) -> Option<Self::Output> {
        input.parse(ParseLineStart)?;
        let mut input = input.start();

        let (indent, kind) = input.parse(ParseBullet)?;
        input.set_line_start(true);
        let ind = self.ind.push(Indentation::spaces(indent));

        let mut content = Vec::new();
        loop {
            dbg!(input.rest());
            let content_parser = Node::multi_parser(ParentKind::List, ind, true);
            content.push(input.parse(content_parser)?);

            let mut input2 = input.start();
            if input2.parse(LineBreak::parser(self.ind)).is_some() {
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
        Some(List { indent, kind, content })
    }
}

struct ParseBullet;

impl Parse for ParseBullet {
    type Output = (u8, ListKind);

    fn parse(&self, input: &mut crate::Input) -> Option<Self::Output> {
        let mut input = input.start();

        let indent = input.parse(WhileChar(' '))?.len();
        if indent > (u8::MAX - 2) as usize {
            return None;
        }
        let indent = indent as u8 + 2;

        let bullet = match input.peek_char() {
            Some('-') => ListKind::Dashes,
            Some('+') => ListKind::Pluses,
            Some('.') => ListKind::Dots,
            Some('*') => ListKind::Stars,
            Some(',') => ListKind::Commas,
            _ => return None,
        };
        input.bump(1);
        input.parse(' ')?;

        input.apply();
        Some((indent, bullet))
    }
}
