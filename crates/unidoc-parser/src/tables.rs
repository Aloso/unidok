use crate::Parse;

use crate::indent::{Indents, LineBreak};
use crate::marker::{ParseLineEnd, ParseLineStart};
use crate::{Node, ParseNodes, UntilChar};

#[derive(Debug, Clone)]
pub struct Table {
    pub eq: usize,
    pub content: Vec<Vec<Vec<Node>>>,
}

pub struct ParseTable<'a> {
    pub ind: Indents<'a>,
}

impl Parse for ParseTable<'_> {
    type Output = Table;

    fn parse(&self, input: &mut crate::Input) -> Option<Self::Output> {
        input.parse(ParseLineStart)?;
        let mut input = input.start();

        input.parse("|===")?;
        let eq = input.parse(UntilChar(|c| c != '='))?.len() + 3;
        input.parse(LineBreak(self.ind))?;

        let mut content = Vec::new();
        loop {
            if input.parse("|===").is_some() {
                let eq_end = input.parse(UntilChar(|c| c != '='))?.len() + 3;
                if eq != eq_end {
                    return None;
                }
                input.parse(ParseLineEnd)?;
                break;
            }

            let mut row = Vec::new();

            while input.parse('|').is_some() {
                let cell = input.parse(ParseNodes {
                    parent: crate::NodeParentKind::Table,
                    ind: self.ind,
                })?;
                row.push(cell);
            }
            input.parse(LineBreak(self.ind))?;
            content.push(row);
        }

        input.apply();
        Some(Table { eq, content })
    }
}
