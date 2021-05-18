use crate::ir::blocks::*;
use crate::ir::html::*;
use crate::ir::macros::Macro;
use crate::ir::segments::*;

pub trait ToPlaintext {
    fn to_plaintext(&self, buf: &mut String);
}

impl ToPlaintext for Segment<'_> {
    fn to_plaintext(&self, buf: &mut String) {
        match self {
            Segment::LineBreak => buf.push('\n'),
            &Segment::HtmlEntity(e) => {
                buf.push('&');
                buf.push_str(e.0);
            }
            Segment::Limiter => {}
            &Segment::Text(t) => buf.push_str(t),
            Segment::Text2(t) => buf.push_str(t),
            &Segment::EscapedText(e) => buf.push_str(e),
            Segment::Braces(b) => b.to_plaintext(buf),
            Segment::Math(m) => m.to_plaintext(buf),
            Segment::Link(l) => l.to_plaintext(buf),
            Segment::Image(i) => i.to_plaintext(buf),
            Segment::InlineHtml(h) => h.to_plaintext(buf),
            Segment::Format(f) => f.to_plaintext(buf),
            Segment::Code(c) => c.to_plaintext(buf),
        }
    }
}

impl ToPlaintext for Braces<'_> {
    fn to_plaintext(&self, buf: &mut String) {
        if self.macros.iter().any(|m| matches!(m, Macro::NoText)) {
            return;
        }
        for s in &self.segments {
            s.to_plaintext(buf);
        }
    }
}

impl ToPlaintext for Math<'_> {
    fn to_plaintext(&self, buf: &mut String) {
        if self.macros.iter().any(|m| matches!(m, Macro::NoText)) {
            return;
        }
        buf.push_str(&self.text);
    }
}

impl ToPlaintext for InlineFormat<'_> {
    fn to_plaintext(&self, buf: &mut String) {
        for s in &self.segments {
            s.to_plaintext(buf);
        }
    }
}

impl ToPlaintext for Code<'_> {
    fn to_plaintext(&self, buf: &mut String) {
        if self.macros.iter().any(|m| matches!(m, Macro::NoText)) {
            return;
        }
        for s in &self.segments {
            s.to_plaintext(buf);
        }
    }
}

impl ToPlaintext for Link<'_> {
    fn to_plaintext(&self, buf: &mut String) {
        if self.macros.iter().any(|m| matches!(m, Macro::NoText)) {
            return;
        }
        for s in &self.text {
            s.to_plaintext(buf);
        }
    }
}

impl ToPlaintext for Image<'_> {
    fn to_plaintext(&self, buf: &mut String) {
        if self.macros.iter().any(|m| matches!(m, Macro::NoText)) {
            return;
        }
        for s in &self.alt {
            s.to_plaintext(buf);
        }
    }
}

impl ToPlaintext for HtmlNode<'_> {
    fn to_plaintext(&self, buf: &mut String) {
        if let HtmlNode::Element(e) = self {
            e.to_plaintext(buf);
        }
    }
}

impl ToPlaintext for HtmlElem<'_> {
    fn to_plaintext(&self, buf: &mut String) {
        if self.macros.iter().any(|m| matches!(m, Macro::NoText)) {
            return;
        }
        if let Some(content) = &self.content {
            match content {
                ElemContent::Blocks(blocks) => {
                    for b in blocks {
                        b.to_plaintext(buf);
                    }
                }
                ElemContent::Inline(segments) => {
                    for s in segments {
                        s.to_plaintext(buf);
                    }
                }
                ElemContent::Verbatim(_) => {}
            }
        }
    }
}

impl ToPlaintext for AnnBlock<'_> {
    fn to_plaintext(&self, buf: &mut String) {
        if self.macros.iter().all(|m| !matches!(m, Macro::NoText)) {
            self.block.to_plaintext(buf);
        }
    }
}

impl ToPlaintext for Block<'_> {
    fn to_plaintext(&self, buf: &mut String) {
        match self {
            Block::CodeBlock(c) => c.to_plaintext(buf),
            Block::Paragraph(p) => p.to_plaintext(buf),
            Block::Heading(h) => h.to_plaintext(buf),
            Block::Table(_) => {} // TODO: Emit warning
            Block::ThematicBreak(_) => buf.push_str("---------\n\n"),
            Block::List(_) => {} // TODO: Emit warning
            Block::Quote(q) => q.to_plaintext(buf),
            Block::Braces(blocks) => {
                for block in blocks {
                    block.to_plaintext(buf)
                }
            }
            Block::BlockHtml(h) => h.to_plaintext(buf),
            Block::Empty => {}
        }
    }
}

impl ToPlaintext for CodeBlock<'_> {
    fn to_plaintext(&self, buf: &mut String) {
        for line in &self.lines {
            line.to_plaintext(buf);
            buf.push('\n');
        }
        buf.push('\n');
    }
}

impl ToPlaintext for Quote<'_> {
    fn to_plaintext(&self, buf: &mut String) {
        for b in &self.content {
            b.to_plaintext(buf);
        }
    }
}

impl ToPlaintext for Paragraph<'_> {
    fn to_plaintext(&self, buf: &mut String) {
        for s in &self.segments {
            s.to_plaintext(buf);
        }
        buf.push('\n');
    }
}

impl ToPlaintext for Heading<'_> {
    fn to_plaintext(&self, buf: &mut String) {
        for s in &self.segments {
            s.to_plaintext(buf);
        }
        buf.push('\n');
    }
}
