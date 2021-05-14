use unidoc_repr::ast::blocks::{CellAlignment, CellMeta, Table, TableCell, TableRow};

use crate::inlines::Segments;
use crate::parsing_mode::ParsingMode;
use crate::utils::{is_ws, ParseLineBreak, ParseLineEnd, ParseSpacesU8, While};
use crate::{Context, Indents, Input, Parse, ParseInfallible};

pub(crate) struct ParseTable<'a> {
    pub ind: Indents<'a>,
}

impl Parse for ParseTable<'_> {
    type Output = Table;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();
        let ind = self.ind;

        input.try_parse(ParseSpacesU8);

        if !input.can_parse("||") && !input.can_parse("#||") {
            return None;
        }

        let mut rows = Vec::new();
        loop {
            if let Some(row) = input.parse(ParseTableRow { ind }) {
                rows.push(row);
            } else {
                return None;
            }
            input.parse(ParseLineEnd)?;

            if input.parse(ParseLineBreak(ind)).is_none() {
                break;
            }

            let mut input2 = input.start();
            input2.try_parse(ParseSpacesU8);

            if !input2.rest().starts_with("||") && !input2.rest().starts_with("#||") {
                break;
            }
            input2.apply();
        }

        input.apply();
        Some(Table { rows })
    }

    fn can_parse(&mut self, input: &mut Input) -> bool {
        let rest = input.rest().trim_start_matches(is_ws);
        rest.starts_with("||") || rest.starts_with("#||")
    }
}

struct ParseTableRow<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseTableRow<'_> {
    type Output = TableRow;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();
        let is_header_row = input.parse('#').is_some();
        input.parse("||")?;

        let mut contents = Vec::new();

        loop {
            let meta = input.parse_i(ParseCellMeta);
            let segments = if matches!(input.peek_char(), Some('\n' | '\r') | None) {
                vec![]
            } else {
                input
                    .parse(Segments::parser(self.ind, Context::Table, ParsingMode::new_all()))?
                    .into_segments_no_underline()?
            };

            contents.push(TableCell { meta, segments });

            if input.parse(ParseLineEnd).is_some() {
                let mut input2 = input.start();

                if input2.parse(ParseLineBreak(self.ind)).is_some()
                    && input2.parse(ParseSpacesU8).is_some()
                    && input2.parse('|').is_some()
                    && !matches!(input2.peek_char(), Some('|'))
                {
                    input2.apply();
                    continue;
                }

                break;
            }

            input.parse('|')?;
        }

        input.apply();
        Some(TableRow { is_header_row, cells: contents })
    }
}

pub(crate) struct ParseCellMeta;

impl ParseInfallible for ParseCellMeta {
    type Output = CellMeta;

    fn parse_infallible(&self, input: &mut Input) -> Self::Output {
        let mut input = input.start();

        let is_header_cell = input.parse('#').is_some();
        let alignment = input.parse_i(ParseCellAlignment);
        let vertical_alignment = input.parse_i(ParseCellAlignment);
        let (colspan, rowspan) = input.parse(ParseRowsAndColumns).unwrap_or((1, 1));

        match input.peek_char() {
            Some(' ' | '\t' | '\n' | '\r') | None => {}
            _ => return CellMeta::default(),
        }

        input.apply();
        CellMeta { is_header_cell, alignment, vertical_alignment, rowspan, colspan }
    }
}

struct ParseCellAlignment;

impl ParseInfallible for ParseCellAlignment {
    type Output = CellAlignment;

    fn parse_infallible(&self, input: &mut Input) -> Self::Output {
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

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
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

        if input.rest().starts_with(is_ws) {
            input.apply();
            Some((col_span.unwrap_or(1), row_span.unwrap_or(1)))
        } else {
            None
        }
    }
}
