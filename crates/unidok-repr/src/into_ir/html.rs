use crate::ast::html::*;
use crate::ast::AstState;
use crate::ir::html::*;
use crate::IntoIR;

use super::utils::collapse_text;

impl<'a> IntoIR<'a> for HtmlNodeAst {
    type IR = HtmlNode<'a>;

    fn into_ir(self, text: &'a str, state: &mut AstState) -> Self::IR {
        match self {
            HtmlNodeAst::Element(e) => HtmlNode::Element(e.into_ir(text, state)),
            HtmlNodeAst::CData(c) => HtmlNode::CData(c.into_ir(text, state)),
            HtmlNodeAst::Comment(c) => HtmlNode::Comment(c.text),
            HtmlNodeAst::Doctype(d) => HtmlNode::Doctype(d.into_ir(text, state)),
        }
    }
}

impl<'a> IntoIR<'a> for DoctypeAst {
    type IR = &'a str;

    fn into_ir(self, text: &'a str, state: &mut AstState) -> Self::IR {
        self.text.into_ir(text, state)
    }
}

impl<'a> IntoIR<'a> for CDataSectionAst {
    type IR = &'a str;

    fn into_ir(self, text: &'a str, state: &mut AstState) -> Self::IR {
        self.text.into_ir(text, state)
    }
}

impl<'a> IntoIR<'a> for HtmlElemAst {
    type IR = HtmlElem<'a>;

    fn into_ir(self, text: &'a str, state: &mut AstState) -> Self::IR {
        HtmlElem {
            macros: vec![],
            name: self.name,
            attrs: self.attrs.into_ir(text, state),
            content: self.content.into_ir(text, state),
            close: self.close,
        }
    }
}

impl<'a> IntoIR<'a> for AttrAst {
    type IR = Attr<'a>;

    fn into_ir(self, text: &'a str, state: &mut AstState) -> Self::IR {
        Attr { key: self.key.into_ir(text, state), value: self.value }
    }
}

impl<'a> IntoIR<'a> for ElemContentAst {
    type IR = ElemContent<'a>;

    fn into_ir(self, text: &'a str, state: &mut AstState) -> Self::IR {
        match self {
            ElemContentAst::Blocks(b) => ElemContent::Blocks(b.into_ir(text, state)),
            ElemContentAst::Inline(i) => ElemContent::Inline(collapse_text(i).into_ir(text, state)),
            ElemContentAst::Verbatim(v) => ElemContent::Verbatim(v),
        }
    }
}
