use unidoc_parser::ir::*;

pub trait ToPlaintext {
    fn to_plaintext(&self, buf: &mut String);
}

impl ToPlaintext for SegmentIr<'_> {
    fn to_plaintext(&self, buf: &mut String) {
        match self {
            SegmentIr::LineBreak => buf.push('\n'),
            SegmentIr::Limiter => {}
            &SegmentIr::Text(t) => buf.push_str(t),
            SegmentIr::Text2(t) => buf.push_str(t),
            &SegmentIr::EscapedText(e) => buf.push_str(e),
            SegmentIr::Braces(b) => b.to_plaintext(buf),
            SegmentIr::Math(m) => m.to_plaintext(buf),
            SegmentIr::Link(l) => l.to_plaintext(buf),
            SegmentIr::Image(i) => i.to_plaintext(buf),
            SegmentIr::InlineMacro(m) => m.to_plaintext(buf),
            SegmentIr::InlineHtml(h) => h.to_plaintext(buf),
            SegmentIr::Format(f) => f.to_plaintext(buf),
            SegmentIr::Code(c) => c.to_plaintext(buf),
        }
    }
}

impl ToPlaintext for BracesIr<'_> {
    fn to_plaintext(&self, buf: &mut String) {
        for s in &self.segments {
            s.to_plaintext(buf);
        }
    }
}

impl ToPlaintext for MathIr {
    fn to_plaintext(&self, buf: &mut String) {
        buf.push_str(&self.text);
    }
}

impl ToPlaintext for InlineFormatIr<'_> {
    fn to_plaintext(&self, buf: &mut String) {
        for s in &self.segments {
            s.to_plaintext(buf);
        }
    }
}

impl ToPlaintext for CodeIr<'_> {
    fn to_plaintext(&self, buf: &mut String) {
        for s in &self.segments {
            s.to_plaintext(buf);
        }
    }
}

impl ToPlaintext for LinkIr<'_> {
    fn to_plaintext(&self, buf: &mut String) {
        for s in &self.text {
            s.to_plaintext(buf);
        }
    }
}

impl ToPlaintext for ImageIr<'_> {
    fn to_plaintext(&self, buf: &mut String) {
        for s in &self.alt {
            s.to_plaintext(buf);
        }
    }
}

impl ToPlaintext for InlineMacroIr<'_> {
    fn to_plaintext(&self, buf: &mut String) {
        self.segment.to_plaintext(buf);
    }
}

impl ToPlaintext for HtmlNodeIr<'_> {
    fn to_plaintext(&self, buf: &mut String) {
        if let HtmlNodeIr::Element(e) = self {
            e.to_plaintext(buf);
        }
    }
}

impl ToPlaintext for HtmlElemIr<'_> {
    fn to_plaintext(&self, buf: &mut String) {
        if let Some(content) = &self.content {
            match content {
                ElemContentIr::Blocks(blocks) => {
                    for b in blocks {
                        b.to_plaintext(buf);
                    }
                }
                ElemContentIr::Inline(segments) => {
                    for s in segments {
                        s.to_plaintext(buf);
                    }
                }
                ElemContentIr::Verbatim(_) => {}
            }
        }
    }
}

impl ToPlaintext for BlockIr<'_> {
    fn to_plaintext(&self, buf: &mut String) {
        match self {
            BlockIr::CodeBlock(c) => c.to_plaintext(buf),
            BlockIr::Paragraph(p) => p.to_plaintext(buf),
            BlockIr::Heading(h) => h.to_plaintext(buf),
            BlockIr::Table(_) => {} // TODO: Emit warning
            BlockIr::ThematicBreak(_) => buf.push_str("---------\n\n"),
            BlockIr::List(_) => {} // TODO: Emit warning
            BlockIr::Quote(q) => q.to_plaintext(buf),
            BlockIr::BlockMacro(m) => m.content.to_plaintext(buf),
            BlockIr::BlockHtml(h) => h.to_plaintext(buf),
            BlockIr::Empty => {}
        }
    }
}

impl ToPlaintext for CodeBlockIr<'_> {
    fn to_plaintext(&self, buf: &mut String) {
        for line in &self.lines {
            line.to_plaintext(buf);
            buf.push('\n');
        }
        buf.push('\n');
    }
}

impl ToPlaintext for QuoteIr<'_> {
    fn to_plaintext(&self, buf: &mut String) {
        for b in &self.content {
            b.to_plaintext(buf);
        }
    }
}

impl ToPlaintext for BlockMacroContentIr<'_> {
    fn to_plaintext(&self, buf: &mut String) {
        match self {
            BlockMacroContentIr::Prefixed(block) => {
                block.to_plaintext(buf);
            }
            BlockMacroContentIr::Braces(blocks) => {
                for block in blocks {
                    block.to_plaintext(buf);
                }
            }
        }
    }
}

impl ToPlaintext for ParagraphIr<'_> {
    fn to_plaintext(&self, buf: &mut String) {
        for s in &self.segments {
            s.to_plaintext(buf);
        }
        buf.push('\n');
    }
}

impl ToPlaintext for HeadingIr<'_> {
    fn to_plaintext(&self, buf: &mut String) {
        for s in &self.segments {
            s.to_plaintext(buf);
        }
        buf.push('\n');
    }
}
