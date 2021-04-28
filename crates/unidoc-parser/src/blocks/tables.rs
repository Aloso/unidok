use crate::inlines::Segment;
use crate::utils::{Indents, ParseLineBreak, ParseLineEnd};
use crate::{Context, Parse, ParseInfallible, StrSlice};

use super::Paragraph;

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
    pub bius: Bius,
    pub css: Vec<StrSlice>,
}

impl Default for CellMeta {
    fn default() -> Self {
        CellMeta {
            is_header_cell: false,
            alignment: CellAlignment::Unset,
            vertical_alignment: CellAlignment::Unset,
            rowspan: 1,
            colspan: 1,
            bius: Bius::new(),
            css: vec![],
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
        if !input.can_parse("||") && !input.can_parse("#||") {
            return None;
        }
        let mut input = input.start();

        let ind = self.ind;
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
        let rest = input.rest();
        rest.starts_with('|') || rest.starts_with("#|")
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
            let meta = input.parse_i(ParseCellMeta);
            let segments = input.parse(Paragraph::parser(self.ind, Context::Table))?.segments;

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

struct ParseCellMeta;

impl ParseInfallible for ParseCellMeta {
    type Output = CellMeta;

    fn parse_infallible(&self, input: &mut crate::Input) -> Self::Output {
        let mut input = input.start();

        let is_header_cell = input.parse('#').is_some();
        let alignment = input.parse_i(ParseCellAlignment);
        let vertical_alignment = input.parse_i(ParseCellAlignment);
        let (rowspan, colspan, bius, css) =
            input.parse(ParseCellMetaBraces).unwrap_or_else(|| (1, 1, Bius::new(), vec![]));

        match input.peek_char() {
            Some(' ' | '\t') => {
                input.bump(1);
            }
            Some('\n') | None => {}
            _ => return CellMeta::default(),
        }

        input.apply();
        CellMeta { is_header_cell, alignment, vertical_alignment, rowspan, colspan, bius, css }
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

struct ParseCellMetaBraces;

impl Parse for ParseCellMetaBraces {
    type Output = (u16, u16, Bius, Vec<StrSlice>);

    fn parse(&self, input: &mut crate::Input) -> Option<Self::Output> {
        let mut input = input.start();
        input.parse('{')?;

        let mut bius = Bius::new();
        let mut alignment: Option<(u16, Option<u16>)> = None;
        let mut css = vec![];

        loop {
            let idx = input.rest().find(|c| matches!(c, ';' | '}'))?;
            if idx > 0 {
                let word = input.bump(idx);
                match word.to_str(input.text()) {
                    "B" => bius = bius.bold(),
                    "I" => bius = bius.italic(),
                    "U" => bius = bius.underline(),
                    "S" => bius = bius.strikethrough(),
                    s => {
                        let mut was_num = false;

                        if let Ok(n) = s.parse::<u16>() {
                            if bius.is_initial() && css.is_empty() {
                                match &mut alignment {
                                    Some((_, Some(_))) => {
                                        return None;
                                    }
                                    Some((_, v @ None)) => {
                                        *v = Some(n);
                                        was_num = true;
                                    }
                                    h @ None => {
                                        *h = Some((n, None));
                                        was_num = true;
                                    }
                                }
                            }
                        }
                        if !was_num {
                            css.push(word);
                        }
                    }
                }
            }

            let next = input.peek_char().unwrap();
            input.bump(1);
            match next {
                ';' => continue,
                '}' => break,
                c => unreachable!("{:?} not expected", c),
            }
        }

        let (hal, val) = alignment.unwrap_or((1, None));
        let val = val.unwrap_or(1);

        input.apply();
        Some((hal, val, bius, css))
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
