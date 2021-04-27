use crate::ir::*;

/// A visitor trait for IR nodes. Create a struct and implement this trait to
/// perform transformations on the IR.
pub trait IrVisitor {
    fn visit_doc(&mut self, doc: &mut DocIr) {
        for block in &mut doc.blocks {
            self.visit_block(block);
        }
    }

    fn visit_block(&mut self, block: &mut BlockIr) {
        match block {
            BlockIr::CodeBlock(b) => self.visit_code_block(b),
            BlockIr::Comment(b) => self.visit_comment(b),
            BlockIr::Paragraph(b) => self.visit_paragraph(b),
            BlockIr::Heading(b) => self.visit_heading(b),
            BlockIr::Table(b) => self.visit_table(b),
            BlockIr::ThematicBreak(b) => self.visit_thematic_break(b),
            BlockIr::List(b) => self.visit_list(b),
            BlockIr::Quote(b) => self.visit_quote(b),
            BlockIr::BlockMacro(b) => self.visit_block_macro(b),
            BlockIr::BlockHtml(h) => self.visit_html_node(h, false),
        }
    }

    fn visit_code_block(&mut self, _: &mut CodeBlockIr) {}

    fn visit_comment(&mut self, _: &mut CommentIr) {}

    fn visit_paragraph(&mut self, paragraph: &mut ParagraphIr) {
        for segment in &mut paragraph.segments {
            self.visit_segment(segment);
        }
    }

    fn visit_heading(&mut self, heading: &mut HeadingIr) {
        for segment in &mut heading.segments {
            self.visit_segment(segment);
        }
    }

    fn visit_table(&mut self, table: &mut TableIr) {
        for row in &mut table.rows {
            self.visit_table_row(row);
        }
    }

    fn visit_table_row(&mut self, table_row: &mut TableRowIr) {
        for cell in &mut table_row.cells {
            self.visit_table_cell(cell);
        }
    }

    fn visit_table_cell(&mut self, table_cell: &mut TableCellIr) {
        for segment in &mut table_cell.segments {
            self.visit_segment(segment);
        }
    }

    fn visit_thematic_break(&mut self, _: &mut ThematicBreakIr) {}

    fn visit_list(&mut self, list: &mut ListIr) {
        for item in &mut list.items {
            self.visit_list_item(item);
        }
    }

    fn visit_list_item(&mut self, list_item: &mut DocIr) {
        for block in &mut list_item.blocks {
            self.visit_block(block);
        }
    }

    fn visit_quote(&mut self, quote: &mut QuoteIr) {
        for block in &mut quote.content.blocks {
            self.visit_block(block);
        }
    }

    fn visit_block_macro(&mut self, mac: &mut BlockMacroIr) {
        match mac {
            BlockMacroIr::AttrMacro { block, .. } => self.visit_block(block),
            BlockMacroIr::BraceMacro { blocks, .. } => {
                for block in blocks {
                    self.visit_block(block);
                }
            }
        }
    }

    fn visit_segment(&mut self, segment: &mut SegmentIr) {
        match segment {
            SegmentIr::Braces(b) => self.visit_braces(b),
            SegmentIr::Math(m) => self.visit_math(m),
            SegmentIr::Link(l) => self.visit_link(l),
            SegmentIr::Image(i) => self.visit_image(i),
            SegmentIr::InlineMacro(m) => self.visit_inline_macro(m),
            SegmentIr::InlineHtml(h) => self.visit_html_node(h, true),
            SegmentIr::Format(f) => self.visit_inline_format(f),
            SegmentIr::Code(c) => self.visit_code(c),

            | SegmentIr::LineBreak
            | SegmentIr::Text(_)
            | SegmentIr::EscapedText(_)
            | SegmentIr::Limiter => {}
        }
    }

    fn visit_braces(&mut self, braces: &mut BracesIr) {
        for segment in &mut braces.segments {
            self.visit_segment(segment);
        }
    }

    fn visit_math(&mut self, _: &mut MathIr) {}

    fn visit_link(&mut self, link: &mut LinkIr) {
        for segment in &mut link.text {
            self.visit_segment(segment);
        }
    }

    fn visit_image(&mut self, image: &mut ImageIr) {
        for segment in &mut image.alt {
            self.visit_segment(segment);
        }
    }

    fn visit_inline_macro(&mut self, mac: &mut InlineMacroIr) {
        self.visit_segment(&mut mac.segment);
    }

    fn visit_inline_format(&mut self, fmt: &mut InlineFormatIr) {
        for segment in &mut fmt.segments {
            self.visit_segment(segment);
        }
    }

    fn visit_code(&mut self, code: &mut CodeIr) {
        for segment in &mut code.segments {
            self.visit_segment(segment);
        }
    }

    fn visit_html_node(&mut self, node: &mut HtmlNodeIr, is_inline: bool) {
        match node {
            HtmlNodeIr::Element(element) => {
                self.visit_html_element(element, is_inline);
            }
            HtmlNodeIr::ClosingTag(_) => {}
            HtmlNodeIr::Cdata(_) => {}
            HtmlNodeIr::Comment(_) => {}
            HtmlNodeIr::Doctype(_) => {}
        }
    }

    fn visit_html_element(&mut self, element: &mut ElementIr, _is_inline: bool) {
        self.visit_html_attributes(&mut element.attrs);

        if let Some(content) = &mut element.content {
            use ElemContentIr::*;
            if let Blocks(blocks) = content {
                for block in blocks {
                    self.visit_block(block);
                }
            }
        }
    }

    fn visit_html_attributes(&mut self, _attrs: &mut Vec<AttrIr>) {}
}
