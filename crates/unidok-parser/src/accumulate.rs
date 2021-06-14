use unidok_repr::ast::blocks::BlockAst;
use unidok_repr::ast::html::{ElemContentAst, HtmlNodeAst};
use unidok_repr::ast::macros::BlockMacroContent;
use unidok_repr::ast::segments::SegmentAst;
use unidok_repr::ast::AstData;

pub(crate) fn accumulate_block_data(
    parsed: &[BlockAst],
    data: &mut AstData,
    no_toc: bool,
    text: &str,
) {
    for parsed in parsed {
        accumulate_single_block_data(parsed, data, no_toc, text)
    }
}

fn accumulate_single_block_data(parsed: &BlockAst, data: &mut AstData, no_toc: bool, text: &str) {
    match parsed {
        BlockAst::CodeBlock(c) => accumulate_block_data(&c.lines, data, no_toc, text),
        BlockAst::Paragraph(p) => accumulate_segment_data(&p.segments, data, no_toc, text),
        BlockAst::Heading(h) => {
            if !no_toc {
                data.headings.push(h.clone());
            }

            accumulate_segment_data(&h.segments, data, no_toc, text)
        }
        BlockAst::Table(t) => {
            for row in &t.rows {
                for cell in &row.cells {
                    accumulate_segment_data(&cell.segments, data, no_toc, text);
                }
            }
        }
        BlockAst::ThematicBreak(_) => {}
        BlockAst::List(l) => {
            for item in &l.items {
                accumulate_block_data(item, data, no_toc, text);
            }
        }
        BlockAst::Quote(q) => accumulate_block_data(&q.content, data, no_toc, text),
        BlockAst::BlockMacro(b) => {
            let no_toc = match b.name.to_str(text) {
                "NOTOC" => true,
                _ => no_toc,
            };
            match &b.content {
                BlockMacroContent::Prefixed(p) => {
                    accumulate_single_block_data(p, data, no_toc, text)
                }
                BlockMacroContent::Braces(b) => accumulate_block_data(b, data, no_toc, text),
                BlockMacroContent::None => {}
            }
        }
        BlockAst::BlockHtml(b) => accumulate_html(b, data, no_toc, text),
        BlockAst::Comment(_) => {}
        BlockAst::LinkRefDef(l) => {
            let name = l.name.to_str(text).to_string();
            data.link_ref_defs.insert(name, l.clone());
        }
    }
}

fn accumulate_segment_data(parsed: &[SegmentAst], data: &mut AstData, no_toc: bool, text: &str) {
    for parsed in parsed {
        accumulate_single_segment_data(parsed, data, no_toc, text)
    }
}

fn accumulate_single_segment_data(
    parsed: &SegmentAst,
    data: &mut AstData,
    no_toc: bool,
    text: &str,
) {
    match parsed {
        SegmentAst::LineBreak => {}
        SegmentAst::Text(_) => {}
        SegmentAst::Text2(_) => {}
        SegmentAst::Text3(_) => {}
        SegmentAst::Escaped(_) => {}
        SegmentAst::Substitution(_) => {}
        SegmentAst::Limiter => {}
        SegmentAst::Braces(b) => {
            accumulate_segment_data(&b.segments, data, no_toc, text);
        }
        SegmentAst::Math(_) => {}
        SegmentAst::Link(l) => {
            if let Some(inner_text) = &l.text {
                accumulate_segment_data(inner_text, data, no_toc, text);
            }
        }
        SegmentAst::Image(i) => {
            if let Some(alt) = &i.alt {
                accumulate_segment_data(alt, data, no_toc, text);
            }
        }
        SegmentAst::InlineMacro(i) => {
            let no_toc = match i.name.to_str(text) {
                "NOTOC" => true,
                _ => no_toc,
            };
            accumulate_single_segment_data(&i.segment, data, no_toc, text);
        }
        SegmentAst::InlineHtml(i) => accumulate_html(i, data, no_toc, text),
        SegmentAst::HtmlEntity(_) => {}
        SegmentAst::Format(f) => accumulate_segment_data(&f.segments, data, no_toc, text),
        SegmentAst::Code(c) => accumulate_segment_data(&c.segments, data, no_toc, text),
    }
}

fn accumulate_html(node: &HtmlNodeAst, data: &mut AstData, no_toc: bool, text: &str) {
    match node {
        HtmlNodeAst::Element(e) => {
            if let Some(c) = &e.content {
                match c {
                    ElemContentAst::Blocks(b) => accumulate_block_data(b, data, no_toc, text),
                    ElemContentAst::Inline(i) => accumulate_segment_data(i, data, no_toc, text),
                    ElemContentAst::Verbatim(_) => {}
                }
            }
        }
        HtmlNodeAst::CData(_) => {}
        HtmlNodeAst::Comment(_) => {}
        HtmlNodeAst::Doctype(_) => {}
    }
}
