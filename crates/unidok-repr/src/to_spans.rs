use crate::ast::blocks::{
    BlockAst, CodeBlockAst, Comment, HeadingAst, LinkRefDef, ListAst, QuoteAst, TableAst,
};
use crate::ast::html::{ElemContentAst, HtmlNodeAst};
use crate::ast::macros::{
    BlockMacro, BlockMacroContent, InlineMacroAst, MacroArgs, TokenTree, TokenTreeAtom,
};
use crate::ast::segments::{
    BracesAst, CodeAst, ImageAst, InlineFormatAst, LinkAst, LinkTarget, SegmentAst,
};
use crate::{Span, SyntaxKind, SyntaxSpan};

pub trait ToSpans {
    fn to_spans(&self, buf: &mut Vec<SyntaxSpan>);
}

impl ToSpans for BlockAst {
    fn to_spans(&self, buf: &mut Vec<SyntaxSpan>) {
        match self {
            BlockAst::CodeBlock(c) => c.to_spans(buf),
            BlockAst::Paragraph(p) => p.segments.to_spans(buf),
            BlockAst::Heading(h) => h.to_spans(buf),
            BlockAst::Table(t) => t.to_spans(buf),
            BlockAst::ThematicBreak(_) => {}
            BlockAst::List(l) => l.to_spans(buf),
            BlockAst::Quote(q) => q.to_spans(buf),
            BlockAst::BlockMacro(b) => b.to_spans(buf),
            BlockAst::BlockHtml(h) => h.to_spans(buf),
            BlockAst::Comment(c) => c.to_spans(buf),
            BlockAst::LinkRefDef(l) => l.to_spans(buf),
        }
    }
}

impl ToSpans for SegmentAst {
    fn to_spans(&self, buf: &mut Vec<SyntaxSpan>) {
        match self {
            SegmentAst::LineBreak
            | SegmentAst::Text(_)
            | SegmentAst::Text2(_)
            | SegmentAst::Text3(_) => {}
            SegmentAst::Escaped(e) => buf.push(Span::from(e.text).with(SyntaxKind::Escaped)),
            SegmentAst::Substitution(_) => {}
            SegmentAst::Limiter => {}
            SegmentAst::Braces(b) => b.to_spans(buf),
            SegmentAst::Math(_) => {}
            SegmentAst::Link(l) => l.to_spans(buf),
            SegmentAst::Image(i) => i.to_spans(buf),
            SegmentAst::InlineMacro(m) => m.to_spans(buf),
            SegmentAst::InlineHtml(i) => i.to_spans(buf),
            SegmentAst::HtmlEntity(_) => {}
            SegmentAst::Format(f) => f.to_spans(buf),
            SegmentAst::Code(c) => c.to_spans(buf),
        }
    }
}

impl ToSpans for [SegmentAst] {
    fn to_spans(&self, buf: &mut Vec<SyntaxSpan>) {
        for segment in self {
            segment.to_spans(buf)
        }
    }
}

impl ToSpans for [BlockAst] {
    fn to_spans(&self, buf: &mut Vec<SyntaxSpan>) {
        for block in self {
            block.to_spans(buf)
        }
    }
}

impl ToSpans for HeadingAst {
    fn to_spans(&self, buf: &mut Vec<SyntaxSpan>) {
        self.segments.to_spans(buf);
    }
}

impl ToSpans for Comment {
    fn to_spans(&self, buf: &mut Vec<SyntaxSpan>) {
        buf.push(Span::from(self.content).with(SyntaxKind::Comment));
    }
}

impl ToSpans for CodeBlockAst {
    fn to_spans(&self, buf: &mut Vec<SyntaxSpan>) {
        let span = self.opening_fence.until(self.closing_fence).with(SyntaxKind::CodeBlock);
        let info = Span::from(self.info).with(SyntaxKind::InfoString);
        buf.push(span);
        buf.push(self.opening_fence.with(SyntaxKind::CodeFence));
        buf.push(info);
        self.lines.to_spans(buf);
        buf.push(self.closing_fence.with(SyntaxKind::CodeFence));
    }
}

impl ToSpans for TableAst {
    fn to_spans(&self, buf: &mut Vec<SyntaxSpan>) {
        for r in &self.rows {
            for c in &r.cells {
                c.segments.to_spans(buf);
            }
        }
    }
}

impl ToSpans for BlockMacro {
    fn to_spans(&self, buf: &mut Vec<SyntaxSpan>) {
        buf.push(Span::from(self.name).with(SyntaxKind::MacroName));
        if let Some(args) = &self.args {
            args.to_spans(buf);
        }
        match &self.content {
            BlockMacroContent::Prefixed(block) => block.to_spans(buf),
            BlockMacroContent::Braces(blocks) => blocks.to_spans(buf),
            BlockMacroContent::None => {}
        }
    }
}

impl ToSpans for ListAst {
    fn to_spans(&self, buf: &mut Vec<SyntaxSpan>) {
        for item in &self.items {
            item.to_spans(buf);
        }
    }
}

impl ToSpans for QuoteAst {
    fn to_spans(&self, buf: &mut Vec<SyntaxSpan>) {
        self.content.to_spans(buf);
    }
}

impl ToSpans for HtmlNodeAst {
    fn to_spans(&self, buf: &mut Vec<SyntaxSpan>) {
        match self {
            HtmlNodeAst::Element(e) => {
                if let Some(content) = &e.content {
                    match content {
                        ElemContentAst::Blocks(blocks) => blocks.to_spans(buf),
                        ElemContentAst::Inline(s) => s.to_spans(buf),
                        ElemContentAst::Verbatim(_) => {}
                    }
                }
            }
            HtmlNodeAst::CData(c) => {
                buf.push(Span::from(c.text).with(SyntaxKind::HtmlCdata));
            }
            HtmlNodeAst::Comment(_) => {}
            HtmlNodeAst::Doctype(d) => {
                buf.push(Span::from(d.text).with(SyntaxKind::HtmlCdata));
            }
        }
    }
}

impl ToSpans for LinkRefDef {
    fn to_spans(&self, buf: &mut Vec<SyntaxSpan>) {
        buf.push(Span::from(self.name).with(SyntaxKind::LinkRef));
        buf.push(Span::from(self.url).with(SyntaxKind::LinkHref));
    }
}

impl ToSpans for BracesAst {
    fn to_spans(&self, buf: &mut Vec<SyntaxSpan>) {
        self.segments.to_spans(buf);
    }
}

impl ToSpans for LinkAst {
    fn to_spans(&self, buf: &mut Vec<SyntaxSpan>) {
        if let Some(text) = &self.text {
            text.to_spans(buf);
        }

        match self.target {
            LinkTarget::Url { href: _, title: _ } => {}
            LinkTarget::Reference(r) => {
                buf.push(Span::from(r).with(SyntaxKind::LinkRef));
            }
            LinkTarget::Footnote => {}
        }
    }
}

impl ToSpans for ImageAst {
    fn to_spans(&self, buf: &mut Vec<SyntaxSpan>) {
        if let Some(text) = &self.alt {
            text.to_spans(buf);
        }

        match self.target {
            LinkTarget::Url { href: _, title: _ } => {}
            LinkTarget::Reference(r) => {
                buf.push(Span::from(r).with(SyntaxKind::LinkRef));
            }
            LinkTarget::Footnote => {}
        }
    }
}

impl ToSpans for InlineMacroAst {
    fn to_spans(&self, buf: &mut Vec<SyntaxSpan>) {
        buf.push(Span::from(self.name).with(SyntaxKind::MacroName));
        if let Some(args) = &self.args {
            args.to_spans(buf);
        }
        self.segment.to_spans(buf);
    }
}

impl ToSpans for InlineFormatAst {
    fn to_spans(&self, buf: &mut Vec<SyntaxSpan>) {
        self.segments.to_spans(buf);
    }
}

impl ToSpans for CodeAst {
    fn to_spans(&self, buf: &mut Vec<SyntaxSpan>) {
        self.segments.to_spans(buf);
    }
}

impl ToSpans for MacroArgs {
    fn to_spans(&self, buf: &mut Vec<SyntaxSpan>) {
        match self {
            &MacroArgs::Raw(r) => {
                buf.push(Span::from(r).with(SyntaxKind::MacroArg));
            }
            MacroArgs::TokenTrees(tts) => {
                for tt in tts {
                    tt.to_spans(buf);
                }
            }
        }
    }
}

impl ToSpans for TokenTree {
    fn to_spans(&self, buf: &mut Vec<SyntaxSpan>) {
        match self {
            TokenTree::Atom(a) => a.to_spans(buf),
            &TokenTree::KV(k, ref v) => {
                buf.push(Span::from(k).with(SyntaxKind::MacroKey));
                v.to_spans(buf);
            }
        }
    }
}

impl ToSpans for TokenTreeAtom {
    fn to_spans(&self, buf: &mut Vec<SyntaxSpan>) {
        match self {
            &TokenTreeAtom::Word(w) => {
                buf.push(Span::from(w).with(SyntaxKind::MacroArg));
            }
            TokenTreeAtom::QuotedWord(_) => {}
            TokenTreeAtom::Tuple(tts) => {
                for tt in tts {
                    tt.to_spans(buf);
                }
            }
            TokenTreeAtom::Braces(b) => b.to_spans(buf),
        }
    }
}
