use crate::ast::blocks::CodeBlockAst;
use crate::{Span, SyntaxKind, SyntaxSpan};

pub trait ToSpans {
    fn to_spans(&self, buf: &mut Vec<SyntaxSpan>);
}

impl ToSpans for CodeBlockAst {
    fn to_spans(&self, buf: &mut Vec<SyntaxSpan>) {
        let span = self.opening_fence.until(self.closing_fence).with(SyntaxKind::CodeBlock);
        let info = Span::from(self.info).with(SyntaxKind::InfoString);
        buf.push(span);
        buf.push(self.opening_fence.with(SyntaxKind::CodeFence));
        buf.push(info);
        // TODO: add spans from children
        buf.push(self.closing_fence.with(SyntaxKind::CodeFence));
    }
}
