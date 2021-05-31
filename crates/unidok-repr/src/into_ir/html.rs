use crate::ast::html::*;
use crate::ast::AstData;
use crate::ir::html::*;
use crate::IntoIR;

use super::utils::collapse_text;

impl<'a> IntoIR<'a> for HtmlNodeAst {
    type IR = HtmlNode<'a>;

    fn into_ir(self, text: &'a str, data: &mut AstData) -> Self::IR {
        match self {
            HtmlNodeAst::Element(e) => HtmlNode::Element(e.into_ir(text, data)),
            HtmlNodeAst::CData(c) => HtmlNode::CData(c.into_ir(text, data)),
            HtmlNodeAst::Comment(c) => HtmlNode::Comment(c.text),
            HtmlNodeAst::Doctype(d) => HtmlNode::Doctype(d.into_ir(text, data)),
        }
    }
}

impl<'a> IntoIR<'a> for DoctypeAst {
    type IR = &'a str;

    fn into_ir(self, text: &'a str, data: &mut AstData) -> Self::IR {
        self.text.into_ir(text, data)
    }
}

impl<'a> IntoIR<'a> for CDataSectionAst {
    type IR = &'a str;

    fn into_ir(self, text: &'a str, data: &mut AstData) -> Self::IR {
        self.text.into_ir(text, data)
    }
}

impl<'a> IntoIR<'a> for HtmlElemAst {
    type IR = HtmlElem<'a>;

    fn into_ir(self, text: &'a str, data: &mut AstData) -> Self::IR {
        HtmlElem {
            macros: vec![],
            name: self.name,
            attrs: self.attrs.into_ir(text, data),
            content: self.content.into_ir(text, data),
            close: self.close,
        }
    }
}

impl<'a> IntoIR<'a> for AttrAst {
    type IR = Attr<'a>;

    fn into_ir(self, text: &'a str, data: &mut AstData) -> Self::IR {
        Attr { key: self.key.into_ir(text, data), value: self.value }
    }
}

impl<'a> IntoIR<'a> for ElemContentAst {
    type IR = ElemContent<'a>;

    fn into_ir(self, text: &'a str, data: &mut AstData) -> Self::IR {
        match self {
            ElemContentAst::Blocks(b) => ElemContent::Blocks(b.into_ir(text, data)),
            ElemContentAst::Inline(i) => ElemContent::Inline(collapse_text(i).into_ir(text, data)),
            ElemContentAst::Verbatim(v) => ElemContent::Verbatim(v),
        }
    }
}
