use crate::ast::blocks::*;
use crate::ast::AstData;
use crate::ir::blocks::*;
use crate::{IntoIR, ToPlaintext};

use super::utils::collapse_text;

impl<'a> IntoIR<'a> for BlockAst {
    type IR = AnnBlock<'a>;

    fn into_ir(self, text: &'a str, data: &mut AstData) -> Self::IR {
        let block = match self {
            BlockAst::CodeBlock(b) => Block::CodeBlock(b.into_ir(text, data)),
            BlockAst::Paragraph(b) => Block::Paragraph(b.into_ir(text, data)),
            BlockAst::Heading(b) => Block::Heading(b.into_ir(text, data)),
            BlockAst::Table(b) => Block::Table(b.into_ir(text, data)),
            BlockAst::ThematicBreak(b) => Block::ThematicBreak(b.into_ir(text, data)),
            BlockAst::List(b) => Block::List(b.into_ir(text, data)),
            BlockAst::Quote(b) => Block::Quote(b.into_ir(text, data)),
            BlockAst::BlockMacro(block) => {
                return block.into_ir(text, data);
            }
            BlockAst::BlockHtml(h) => Block::BlockHtml(h.into_ir(text, data)),

            BlockAst::Comment(_) | BlockAst::LinkRefDef(_) => Block::Empty,
        };
        AnnBlock { macros: vec![], block }
    }
}

impl<'a> IntoIR<'a> for CodeBlockAst {
    type IR = CodeBlock<'a>;

    fn into_ir(self, text: &'a str, data: &mut AstData) -> Self::IR {
        let lines = self
            .lines
            .into_iter()
            .map(|b| {
                let b = b.into_ir(text, data);
                debug_assert!(b.macros.is_empty());
                b.block
            })
            .collect();

        CodeBlock {
            info: self.info.into_ir(text, data),
            fence: self.fence_type,
            lines,
            indent: self.indent,
        }
    }
}

impl<'a> IntoIR<'a> for ParagraphAst {
    type IR = Paragraph<'a>;

    fn into_ir(self, text: &'a str, data: &mut AstData) -> Self::IR {
        Paragraph { segments: collapse_text(self.segments).into_ir(text, data) }
    }
}

impl<'a> IntoIR<'a> for HeadingAst {
    type IR = Heading<'a>;

    fn into_ir(self, text: &'a str, data: &mut AstData) -> Self::IR {
        let segments = collapse_text(self.segments).into_ir(text, data);

        let mut plaintext = String::new();
        for segment in &segments {
            segment.to_plaintext(&mut plaintext);
        }
        let slug = slug::slugify(plaintext);

        Heading { level: self.level, segments, slug }
    }
}

impl<'a> IntoIR<'a> for ThematicBreakAst {
    type IR = ThematicBreak;

    fn into_ir(self, _: &str, _: &mut AstData) -> Self::IR {
        ThematicBreak { len: self.len, kind: self.kind }
    }
}

impl<'a> IntoIR<'a> for TableAst {
    type IR = Table<'a>;

    fn into_ir(self, text: &'a str, data: &mut AstData) -> Self::IR {
        Table { rows: self.rows.into_ir(text, data) }
    }
}

impl<'a> IntoIR<'a> for TableRowAst {
    type IR = TableRow<'a>;

    fn into_ir(self, text: &'a str, data: &mut AstData) -> Self::IR {
        TableRow { is_header_row: self.is_header_row, cells: self.cells.into_ir(text, data) }
    }
}

impl<'a> IntoIR<'a> for TableCellAst {
    type IR = TableCell<'a>;

    fn into_ir(self, text: &'a str, data: &mut AstData) -> Self::IR {
        TableCell {
            meta: self.meta.into_ir(text, data),
            segments: collapse_text(self.segments).into_ir(text, data),
        }
    }
}

impl<'a> IntoIR<'a> for CellMetaAst {
    type IR = CellMeta;

    fn into_ir(self, _: &'a str, _: &mut AstData) -> Self::IR {
        CellMeta {
            is_header_cell: self.is_header_cell,
            alignment: self.alignment,
            vertical_alignment: self.vertical_alignment,
            rowspan: self.rowspan,
            colspan: self.colspan,
        }
    }
}

impl<'a> IntoIR<'a> for ListAst {
    type IR = List<'a>;

    fn into_ir(self, text: &'a str, data: &mut AstData) -> Self::IR {
        List { bullet: self.bullet, items: self.items.into_ir(text, data), macros: vec![] }
    }
}

impl<'a> IntoIR<'a> for QuoteAst {
    type IR = Quote<'a>;

    fn into_ir(self, text: &'a str, data: &mut AstData) -> Self::IR {
        Quote { content: self.content.into_ir(text, data) }
    }
}
