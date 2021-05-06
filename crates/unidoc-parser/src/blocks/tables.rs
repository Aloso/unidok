use crate::inlines::segments::Segments;
use crate::inlines::Segment;
use crate::parsing_mode::ParsingMode;
use crate::utils::{Indents, ParseLineBreak, ParseLineEnd, ParseSpacesU8, While};
use crate::{Context, Parse, ParseInfallible};

#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    pub rows: Vec<TableRow>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableRow {
    pub is_header_row: bool,
    pub cells: Vec<TableCell>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableCell {
    pub meta: CellMeta,
    pub segments: Vec<Segment>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CellMeta {
    pub is_header_cell: bool,
    pub alignment: CellAlignment,
    pub vertical_alignment: CellAlignment,
    pub rowspan: u16,
    pub colspan: u16,
}

impl CellMeta {
    pub(crate) fn parser() -> ParseCellMeta {
        ParseCellMeta
    }
}

impl Default for CellMeta {
    fn default() -> Self {
        CellMeta {
            is_header_cell: false,
            alignment: CellAlignment::Unset,
            vertical_alignment: CellAlignment::Unset,
            rowspan: 1,
            colspan: 1,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CellAlignment {
    Unset,
    LeftTop,
    RightBottom,
    Center,
}

impl Table {
    pub(crate) fn parser(ind: Indents<'_>) -> ParseTable<'_> {
        ParseTable { ind }
    }
}

pub(crate) struct ParseTable<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseTable<'_> {
    type Output = Table;

    fn parse(&self, input: &mut crate::Input) -> Option<Self::Output> {
        let mut input = input.start();
        let ind = self.ind.push_indent(input.parse(ParseSpacesU8)?);

        if !input.can_parse("||") && !input.can_parse("#||") {
            return None;
        }

        let mut rows = Vec::new();
        loop {
            if let Some(row) = input.parse(ParseRow { ind }) {
                rows.push(row);
            } else {
                return None;
            }
            input.parse(ParseLineEnd)?;
            if input.parse(ParseLineBreak(ind)).is_some()
                && !input.rest().starts_with("||")
                && !input.rest().starts_with("#||")
            {
                break;
            }
        }

        input.apply();
        Some(Table { rows })
    }

    fn can_parse(&self, input: &mut crate::Input) -> bool {
        let rest = input.rest().trim_start_matches(|c| matches!(c, ' ' | '\t'));
        rest.starts_with("||") || rest.starts_with("#||")
    }
}

pub struct ParseRow<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseRow<'_> {
    type Output = TableRow;

    fn parse(&self, input: &mut crate::Input) -> Option<Self::Output> {
        let mut input = input.start();
        let is_header_row = input.parse('#').is_some();
        input.parse("||")?;

        let mut contents = Vec::new();

        loop {
            let meta = input.parse_i(CellMeta::parser());
            let segments = input
                .parse(Segments::parser(self.ind, Context::Table, ParsingMode::new_all()))?
                .into_segments_no_underline()?;

            contents.push(TableCell { meta, segments });

            if input.parse(ParseLineEnd).is_some() {
                break;
            }

            input.parse('|')?;

            if input.parse(ParseLineEnd).is_some() {
                break;
            }
        }

        input.apply();
        Some(TableRow { is_header_row, cells: contents })
    }
}

pub(crate) struct ParseCellMeta;

impl ParseInfallible for ParseCellMeta {
    type Output = CellMeta;

    fn parse_infallible(&self, input: &mut crate::Input) -> Self::Output {
        let mut input = input.start();

        let is_header_cell = input.parse('#').is_some();
        let alignment = input.parse_i(ParseCellAlignment);
        let vertical_alignment = input.parse_i(ParseCellAlignment);
        let (colspan, rowspan) = input.parse(ParseRowsAndColumns).unwrap_or((1, 1));

        match input.peek_char() {
            Some(' ' | '\t') | Some('\n') | None => {}
            _ => return CellMeta::default(),
        }

        input.apply();
        CellMeta { is_header_cell, alignment, vertical_alignment, rowspan, colspan }
    }
}

struct ParseCellAlignment;

impl ParseInfallible for ParseCellAlignment {
    type Output = CellAlignment;

    fn parse_infallible(&self, input: &mut crate::Input) -> Self::Output {
        let al = match input.peek_char() {
            Some('<') => CellAlignment::LeftTop,
            Some('>') => CellAlignment::RightBottom,
            Some('^') => CellAlignment::Center,
            _ => return CellAlignment::Unset,
        };
        input.bump(1);
        al
    }
}

struct ParseRowsAndColumns;

impl Parse for ParseRowsAndColumns {
    type Output = (u16, u16);

    fn parse(&self, input: &mut crate::Input) -> Option<Self::Output> {
        let mut input = input.start();

        let mut col_span: Option<u16> = None;
        let mut row_span: Option<u16> = None;

        let num = input.parse_i(While(|c: char| c.is_ascii_digit()));
        if !num.is_empty() {
            let num = num.to_str(input.text()).parse().ok()?;
            col_span = Some(num);
        }
        if input.parse('x').is_some() {
            let num = input.parse_i(While(|c: char| c.is_ascii_digit()));
            if !num.is_empty() {
                let num = num.to_str(input.text()).parse().ok()?;
                row_span = Some(num);
            }
        }

        if input.rest().starts_with(|c: char| matches!(c, ' ' | '\t')) {
            input.apply();
            Some((col_span.unwrap_or(1), row_span.unwrap_or(1)))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bius(u8);

impl Bius {
    const BOLD: u8 = 1;
    const ITALIC: u8 = 2;
    const UNDERLINE: u8 = 4;
    const STRIKE: u8 = 8;

    pub fn new() -> Bius {
        Bius(0)
    }

    pub fn bold(mut self) -> Bius {
        self.0 |= Bius::BOLD;
        self
    }

    pub fn italic(mut self) -> Bius {
        self.0 |= Bius::ITALIC;
        self
    }

    pub fn underline(mut self) -> Bius {
        self.0 |= Bius::UNDERLINE;
        self
    }

    pub fn strikethrough(mut self) -> Bius {
        self.0 |= Bius::STRIKE;
        self
    }

    pub fn is_initial(&self) -> bool {
        self.0 == 0
    }

    pub fn is_bold(&self) -> bool {
        self.0 & Bius::BOLD != 0
    }

    pub fn is_italic(&self) -> bool {
        self.0 & Bius::ITALIC != 0
    }

    pub fn is_underline(&self) -> bool {
        self.0 & Bius::UNDERLINE != 0
    }

    pub fn is_strikethrough(&self) -> bool {
        self.0 & Bius::STRIKE != 0
    }
}

impl Default for Bius {
    fn default() -> Self {
        Self::new()
    }
}
