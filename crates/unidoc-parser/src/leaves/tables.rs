use crate::indent::Indents;
use crate::inlines::{Segment, SegmentCtx};
use crate::items::LineBreak;
use crate::utils::{ParseLineEnd, ParseLineStart};
use crate::{Parse, UntilChar};

#[derive(Debug, Clone)]
pub struct Table {
    pub eq: usize,
    pub content: Vec<Vec<Vec<Segment>>>,
}

impl Table {
    pub fn parser(ind: Indents<'_>) -> ParseTable<'_> {
        ParseTable { ind }
    }
}

pub struct ParseTable<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseTable<'_> {
    type Output = Table;

    fn parse(&self, input: &mut crate::Input) -> Option<Self::Output> {
        input.parse(ParseLineStart)?;
        let mut input = input.start();

        input.parse("|===")?;
        let eq = input.parse(UntilChar(|c| c != '='))?.len() + 3;
        input.parse(LineBreak::parser(self.ind))?;

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
                let cell_parser = Segment::multi_parser(SegmentCtx::Table, self.ind);
                row.push(input.parse(cell_parser)?);
            }
            input.parse(LineBreak::parser(self.ind))?;
            content.push(row);
        }

        input.apply();
        Some(Table { eq, content })
    }
}
