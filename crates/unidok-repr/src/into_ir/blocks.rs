use crate::ast::blocks::*;
use crate::ast::AstState;
use crate::ir::blocks::*;
use crate::{IntoIR, ToPlaintext};

use super::utils::collapse_text;

impl<'a> IntoIR<'a> for Block {
    type IR = AnnBlockIr<'a>;

    fn into_ir(self, text: &'a str, state: &mut AstState) -> Self::IR {
        let block = match self {
            Block::CodeBlock(b) => BlockIr::CodeBlock(b.into_ir(text, state)),
            Block::Paragraph(b) => BlockIr::Paragraph(b.into_ir(text, state)),
            Block::Heading(b) => BlockIr::Heading(b.into_ir(text, state)),
            Block::Table(b) => BlockIr::Table(b.into_ir(text, state)),
            Block::ThematicBreak(b) => BlockIr::ThematicBreak(b.into_ir(text, state)),
            Block::List(b) => BlockIr::List(b.into_ir(text, state)),
            Block::Quote(b) => BlockIr::Quote(b.into_ir(text, state)),
            Block::BlockMacro(block) => {
                return block.into_ir(text, state);
            }
            Block::BlockHtml(h) => BlockIr::BlockHtml(h.into_ir(text, state)),

            Block::Comment(_) | Block::LinkRefDef(_) => BlockIr::Empty,
        };
        AnnBlockIr { macros: vec![], block }
    }
}

impl<'a> IntoIR<'a> for CodeBlock {
    type IR = CodeBlockIr<'a>;

    fn into_ir(self, text: &'a str, state: &mut AstState) -> Self::IR {
        let lines = self
            .lines
            .into_iter()
            .map(|b| {
                let b = b.into_ir(text, state);
                debug_assert!(b.macros.is_empty());
                b.block
            })
            .collect();

        CodeBlockIr {
            info: self.info.into_ir(text, state),
            fence: self.fence,
            lines,
            indent: self.indent,
        }
    }
}

impl<'a> IntoIR<'a> for Paragraph {
    type IR = ParagraphIr<'a>;

    fn into_ir(self, text: &'a str, state: &mut AstState) -> Self::IR {
        ParagraphIr { segments: collapse_text(self.segments).into_ir(text, state) }
    }
}

impl<'a> IntoIR<'a> for Heading {
    type IR = HeadingIr<'a>;

    fn into_ir(self, text: &'a str, state: &mut AstState) -> Self::IR {
        let segments = collapse_text(self.segments).into_ir(text, state);

        let mut plaintext = String::new();
        for segment in &segments {
            segment.to_plaintext(&mut plaintext);
        }
        let slug = slug::slugify(plaintext);

        HeadingIr { level: self.level, segments, slug }
    }
}

impl<'a> IntoIR<'a> for ThematicBreak {
    type IR = ThematicBreakIr;

    fn into_ir(self, _: &str, _: &mut AstState) -> Self::IR {
        ThematicBreakIr { len: self.len, kind: self.kind }
    }
}

impl<'a> IntoIR<'a> for Table {
    type IR = TableIr<'a>;

    fn into_ir(self, text: &'a str, state: &mut AstState) -> Self::IR {
        TableIr { rows: self.rows.into_ir(text, state) }
    }
}

impl<'a> IntoIR<'a> for TableRow {
    type IR = TableRowIr<'a>;

    fn into_ir(self, text: &'a str, state: &mut AstState) -> Self::IR {
        TableRowIr { is_header_row: self.is_header_row, cells: self.cells.into_ir(text, state) }
    }
}

impl<'a> IntoIR<'a> for TableCell {
    type IR = TableCellIr<'a>;

    fn into_ir(self, text: &'a str, state: &mut AstState) -> Self::IR {
        TableCellIr {
            meta: self.meta.into_ir(text, state),
            segments: collapse_text(self.segments).into_ir(text, state),
        }
    }
}

impl<'a> IntoIR<'a> for CellMeta {
    type IR = CellMetaIr;

    fn into_ir(self, _: &'a str, _: &mut AstState) -> Self::IR {
        CellMetaIr {
            is_header_cell: self.is_header_cell,
            alignment: self.alignment,
            vertical_alignment: self.vertical_alignment,
            rowspan: self.rowspan,
            colspan: self.colspan,
        }
    }
}

impl<'a> IntoIR<'a> for List {
    type IR = ListIr<'a>;

    fn into_ir(self, text: &'a str, state: &mut AstState) -> Self::IR {
        ListIr { bullet: self.bullet, items: self.items.into_ir(text, state), macros: vec![] }
    }
}

impl<'a> IntoIR<'a> for Quote {
    type IR = QuoteIr<'a>;

    fn into_ir(self, text: &'a str, state: &mut AstState) -> Self::IR {
        QuoteIr { content: self.content.into_ir(text, state) }
    }
}
