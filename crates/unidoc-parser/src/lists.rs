use std::convert::TryFrom;

use crate::indent::{Indentation, Indents};
use crate::items::{LineBreak, Node, ParentKind};
use crate::marker::ParseLineStart;
use crate::{Parse, UntilChar};

#[derive(Debug, Clone)]
pub struct List {
    pub indent: u8,
    pub kind: ListKind,
    pub content: Vec<Vec<Node>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListKind {
    Dashes,
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
    pub ind: Indents<'a>,
}

impl Parse for ParseList<'_> {
    type Output = List;

    fn parse(&self, input: &mut crate::Input) -> Option<Self::Output> {
        input.parse(ParseLineStart)?;
        let mut input = input.start();

        let (indent, kind) = input.parse(ParseBullet)?;
        input.set_line_start(true);
        let ind = self.ind.push(Indentation::spaces(indent + 2));

        let mut content = Vec::new();
        loop {
            let node = input.parse(Node::multi_parser(ParentKind::List, ind))?;
            content.push(node);

            let mut input2 = input.start();
            if input2.parse(LineBreak::parser(self.ind)).is_some() {
                let (indent2, kind2) = input2.parse(ParseBullet)?;
                if indent2 == indent && kind2 == kind {
                    input2.apply();
                    continue;
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

        let indent = input.parse(UntilChar(|c| c != ' '))?.len();
        let bullet = match input.peek_char() {
            Some('-') => ListKind::Dashes,
            Some('.') => ListKind::Dots,
            Some('*') => ListKind::Stars,
            Some(',') => ListKind::Commas,
            _ => return None,
        };
        input.bump(1);
        input.parse(' ')?;

        input.apply();
        Some((u8::try_from(indent).ok()? + 2, bullet))
    }
}
