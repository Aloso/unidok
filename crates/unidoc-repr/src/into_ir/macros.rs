use detached_str::StrSlice;

use crate::ast::macros::*;
use crate::ast::AstState;
use crate::ir::blocks::{AnnBlockIr, AnnotationIr, BlockIr};
use crate::ir::html::HtmlNodeIr;
use crate::ir::macros::*;
use crate::ir::segments::SegmentIr;
use crate::IntoIR;

impl<'a> IntoIR<'a> for BlockMacro {
    type IR = AnnBlockIr<'a>;

    fn into_ir(self, text: &'a str, state: &AstState) -> Self::IR {
        let mut block = self.content.into_ir(text, state);
        block.annotations.push(AnnotationIr {
            name: self.name.into_ir(text, state),
            args: self.args.into_ir(text, state),
        });
        block
    }
}

impl<'a> IntoIR<'a> for InlineMacro {
    type IR = SegmentIr<'a>;

    fn into_ir(self, text: &'a str, state: &AstState) -> Self::IR {
        let mut segment = (*self.segment).into_ir(text, state);
        match &mut segment {
            SegmentIr::Braces(b) => {
                add_annotation(&mut b.annotations, self.name, self.args, text, state);
            }
            SegmentIr::Math(b) => {
                add_annotation(&mut b.annotations, self.name, self.args, text, state);
            }
            SegmentIr::Link(b) => {
                add_annotation(&mut b.annotations, self.name, self.args, text, state);
            }
            SegmentIr::Image(b) => {
                add_annotation(&mut b.annotations, self.name, self.args, text, state);
            }
            SegmentIr::Code(b) => {
                add_annotation(&mut b.annotations, self.name, self.args, text, state);
            }
            SegmentIr::InlineHtml(HtmlNodeIr::Element(b)) => {
                add_annotation(&mut b.annotations, self.name, self.args, text, state);
            }

            _ => {}
        }
        segment
    }
}

fn add_annotation<'a>(
    annotations: &mut Vec<AnnotationIr<'a>>,
    name: StrSlice,
    args: Option<MacroArgs>,
    text: &'a str,
    state: &AstState,
) {
    annotations.push(AnnotationIr { name: name.to_str(text), args: args.into_ir(text, state) });
}

impl<'a> IntoIR<'a> for BlockMacroContent {
    type IR = AnnBlockIr<'a>;

    fn into_ir(self, text: &'a str, state: &AstState) -> Self::IR {
        match self {
            BlockMacroContent::Prefixed(p) => (*p).into_ir(text, state),
            BlockMacroContent::Braces(b) => {
                AnnBlockIr { annotations: vec![], block: BlockIr::Braces(b.into_ir(text, state)) }
            }
        }
    }
}

impl<'a> IntoIR<'a> for MacroArgs {
    type IR = MacroArgsIr<'a>;

    fn into_ir(self, text: &'a str, state: &AstState) -> Self::IR {
        match self {
            MacroArgs::Raw(r) => MacroArgsIr::Raw(r.into_ir(text, state)),
            MacroArgs::TokenTrees(t) => MacroArgsIr::TokenTrees(t.into_ir(text, state)),
        }
    }
}

impl<'a> IntoIR<'a> for TokenTree {
    type IR = TokenTreeIr<'a>;

    fn into_ir(self, text: &'a str, state: &AstState) -> Self::IR {
        match self {
            TokenTree::Atom(a) => TokenTreeIr::Atom(a.into_ir(text, state)),
            TokenTree::KV(k, v) => TokenTreeIr::KV(k.into_ir(text, state), v.into_ir(text, state)),
        }
    }
}

impl<'a> IntoIR<'a> for TokenTreeAtom {
    type IR = TokenTreeAtomIr<'a>;

    fn into_ir(self, text: &'a str, state: &AstState) -> Self::IR {
        match self {
            TokenTreeAtom::Word(w) => TokenTreeAtomIr::Word(w.into_ir(text, state)),
            TokenTreeAtom::QuotedWord(q) => TokenTreeAtomIr::QuotedWord(q),
            TokenTreeAtom::Tuple(t) => TokenTreeAtomIr::Tuple(t.into_ir(text, state)),
            TokenTreeAtom::Braces(b) => TokenTreeAtomIr::Braces(b.into_ir(text, state)),
        }
    }
}
