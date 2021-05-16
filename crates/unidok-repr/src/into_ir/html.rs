use crate::ast::html::*;
use crate::ast::AstState;
use crate::ir::html::*;
use crate::IntoIR;

use super::utils::collapse_text;

impl<'a> IntoIR<'a> for HtmlNode {
    type IR = HtmlNodeIr<'a>;

    fn into_ir(self, text: &'a str, state: &mut AstState) -> Self::IR {
        match self {
            HtmlNode::Element(e) => HtmlNodeIr::Element(e.into_ir(text, state)),
            HtmlNode::CData(c) => HtmlNodeIr::CData(c.into_ir(text, state)),
            HtmlNode::Comment(c) => HtmlNodeIr::Comment(c.text),
            HtmlNode::Doctype(d) => HtmlNodeIr::Doctype(d.into_ir(text, state)),
        }
    }
}

impl<'a> IntoIR<'a> for Doctype {
    type IR = &'a str;

    fn into_ir(self, text: &'a str, state: &mut AstState) -> Self::IR {
        self.text.into_ir(text, state)
    }
}

impl<'a> IntoIR<'a> for CDataSection {
    type IR = &'a str;

    fn into_ir(self, text: &'a str, state: &mut AstState) -> Self::IR {
        self.text.into_ir(text, state)
    }
}

impl<'a> IntoIR<'a> for HtmlElem {
    type IR = HtmlElemIr<'a>;

    fn into_ir(self, text: &'a str, state: &mut AstState) -> Self::IR {
        HtmlElemIr {
            macros: vec![],
            name: self.name,
            attrs: self.attrs.into_ir(text, state),
            content: self.content.into_ir(text, state),
            close: self.close,
        }
    }
}

impl<'a> IntoIR<'a> for HtmlAttr {
    type IR = AttrIr<'a>;

    fn into_ir(self, text: &'a str, state: &mut AstState) -> Self::IR {
        AttrIr { key: self.key.into_ir(text, state), value: self.value }
    }
}

impl<'a> IntoIR<'a> for ElemContent {
    type IR = ElemContentIr<'a>;

    fn into_ir(self, text: &'a str, state: &mut AstState) -> Self::IR {
        match self {
            ElemContent::Blocks(b) => ElemContentIr::Blocks(b.into_ir(text, state)),
            ElemContent::Inline(i) => ElemContentIr::Inline(collapse_text(i).into_ir(text, state)),
            ElemContent::Verbatim(v) => ElemContentIr::Verbatim(v),
        }
    }
}
