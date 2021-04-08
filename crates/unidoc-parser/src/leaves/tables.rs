use crate::inlines::{Segment, SegmentCtx};
use crate::utils::{If, Indents, ParseLineBreak, ParseLineEnd, WhileChar};
use crate::Parse;

#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    pub content: Vec<TableRow>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TableRow {
    Content(Vec<Vec<Segment>>),
    Line(Vec<ColumnKind>),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ColumnKind {
    Normal,
    AlignLeft,
    AlignRight,
    AlignCenter,
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
        input.parse(If(input.rest().starts_with('|')));
        let mut input = input.start();

        let ind = self.ind;
        let mut content = Vec::new();
        loop {
            if let Some(row) = input.parse(ParseLineRow) {
                content.push(row);
            } else if let Some(row) = input.parse(ParseContentRow { ind }) {
                content.push(row);
            } else {
                return None;
            }
            input.parse(ParseLineBreak(ind))?;
            if !input.rest().starts_with('|') {
                break;
            }
        }

        input.apply();
        Some(Table { content })
    }

    fn can_parse(&self, input: &mut crate::Input) -> bool {
        input.rest().starts_with('|')
    }
}

pub struct ParseContentRow<'a> {
    ind: Indents<'a>,
}

pub struct ParseLineRow;

impl Parse for ParseContentRow<'_> {
    type Output = TableRow;

    fn parse(&self, input: &mut crate::Input) -> Option<Self::Output> {
        let mut input = input.start();
        input.parse('|')?;

        let mut contents = Vec::new();

        loop {
            let content =
                input.parse(Segment::multi_parser(SegmentCtx::Table, self.ind))?;

            contents.push(content);

            if input.parse(ParseLineEnd).is_some() {
                break;
            }

            input.parse('|')?;

            if input.parse(ParseLineEnd).is_some() {
                break;
            }
        }

        Some(TableRow::Content(contents))
    }
}

impl Parse for ParseLineRow {
    type Output = TableRow;

    fn parse(&self, input: &mut crate::Input) -> Option<Self::Output> {
        let mut input = input.start();
        input.parse('|')?;

        if !matches!(input.peek_char(), Some('-' | ':')) {
            return None;
        }

        let mut kinds = Vec::new();

        loop {
            let colon1 = input.parse(':').is_some();
            input.parse("---")?;
            input.parse(WhileChar('-'))?;
            let colon2 = input.parse(':').is_some();

            kinds.push(match (colon1, colon2) {
                (false, false) => ColumnKind::Normal,
                (true, false) => ColumnKind::AlignLeft,
                (false, true) => ColumnKind::AlignRight,
                (true, true) => ColumnKind::AlignCenter,
            });

            if input.parse(ParseLineEnd).is_some() {
                break;
            }

            input.parse('|')?;

            if input.parse(ParseLineEnd).is_some() {
                break;
            }
        }

        Some(TableRow::Line(kinds))
    }
}
