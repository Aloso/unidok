use std::iter;

use detached_str::StrSlice;

use crate::ast::macros::*;
use crate::ast::AstState;
use crate::ir::blocks::{AnnBlockIr, BlockIr};
use crate::ir::html::HtmlNodeIr;
use crate::ir::macros::{Attr, AttrValue, MacroIr};
use crate::ir::segments::SegmentIr;
use crate::IntoIR;

impl<'a> IntoIR<'a> for BlockMacro {
    type IR = AnnBlockIr<'a>;

    fn into_ir(self, text: &'a str, state: &AstState) -> Self::IR {
        let mut block = self.content.into_ir(text, state);
        let r#macro = Macro { name: self.name, args: self.args }.into_ir(text, state);

        if r#macro.is_for_list() {
            if let AnnBlockIr { block: BlockIr::List(list), .. } = &mut block {
                list.macros.push(r#macro);
            }
        } else {
            block.macros.push(r#macro);
        }

        block
    }
}

impl<'a> IntoIR<'a> for InlineMacro {
    type IR = SegmentIr<'a>;

    fn into_ir(self, text: &'a str, state: &AstState) -> Self::IR {
        let mut segment = (*self.segment).into_ir(text, state);
        let r#macro = Macro { name: self.name, args: self.args };
        match &mut segment {
            SegmentIr::Braces(b) => b.macros.push(r#macro.into_ir(text, state)),
            SegmentIr::Math(b) => b.macros.push(r#macro.into_ir(text, state)),
            SegmentIr::Link(b) => b.macros.push(r#macro.into_ir(text, state)),
            SegmentIr::Image(b) => b.macros.push(r#macro.into_ir(text, state)),
            SegmentIr::Code(b) => b.macros.push(r#macro.into_ir(text, state)),
            SegmentIr::InlineHtml(HtmlNodeIr::Element(b)) => {
                b.macros.push(r#macro.into_ir(text, state))
            }

            _ => {}
        }
        segment
    }
}

impl<'a> IntoIR<'a> for BlockMacroContent {
    type IR = AnnBlockIr<'a>;

    fn into_ir(self, text: &'a str, state: &AstState) -> Self::IR {
        match self {
            BlockMacroContent::Prefixed(p) => (*p).into_ir(text, state),
            BlockMacroContent::Braces(b) => {
                AnnBlockIr { macros: vec![], block: BlockIr::Braces(b.into_ir(text, state)) }
            }
        }
    }
}

struct Macro {
    name: StrSlice,
    args: Option<MacroArgs>,
}

impl<'a> IntoIR<'a> for Macro {
    type IR = MacroIr<'a>;

    fn into_ir(self, text: &'a str, _: &AstState) -> Self::IR {
        match self.name.to_str(text) {
            "" => {
                if let Some(MacroArgs::TokenTrees(tts)) = self.args {
                    if tts.is_empty() {
                        return MacroIr::Invalid;
                    }
                    let mut result = Vec::new();

                    for tt in tts {
                        match tt {
                            TokenTree::Atom(TokenTreeAtom::Word(arg)) => {
                                let arg = arg.to_str(text);
                                if let Some(arg) = arg.strip_prefix('.') {
                                    result.push(Attr {
                                        key: "class",
                                        value: Some(AttrValue::Word(arg)),
                                    });
                                } else if let Some(arg) = arg.strip_prefix('#') {
                                    result.push(Attr {
                                        key: "id",
                                        value: Some(AttrValue::Word(arg)),
                                    });
                                } else {
                                    result.push(Attr { key: arg, value: None })
                                }
                            }
                            TokenTree::Atom(TokenTreeAtom::QuotedWord(word)) => result.push(Attr {
                                key: "style",
                                value: Some(AttrValue::QuotedWord(word)),
                            }),
                            TokenTree::KV(key, TokenTreeAtom::Word(word)) => {
                                let key = key.to_str(text);
                                let word = word.to_str(text);
                                result.push(Attr { key, value: Some(AttrValue::Word(word)) })
                            }
                            TokenTree::KV(key, TokenTreeAtom::QuotedWord(word)) => {
                                let key = key.to_str(text);
                                result.push(Attr { key, value: Some(AttrValue::QuotedWord(word)) })
                            }
                            _ => return MacroIr::Invalid,
                        }
                    }

                    MacroIr::HtmlAttrs(result)
                } else {
                    MacroIr::Invalid
                }
            }
            "TOC" => {
                if self.args.is_none() {
                    MacroIr::Toc
                } else {
                    MacroIr::Invalid
                }
            }
            "NOTOC" => {
                if self.args.is_none() {
                    MacroIr::NoToc
                } else {
                    MacroIr::Invalid
                }
            }
            "NOTXT" => {
                if self.args.is_none() {
                    MacroIr::NoText
                } else {
                    MacroIr::Invalid
                }
            }
            "LOOSE" => {
                if self.args.is_none() {
                    MacroIr::Loose
                } else {
                    MacroIr::Invalid
                }
            }
            "BULLET" => {
                if let Some(MacroArgs::TokenTrees(tts)) = self.args {
                    if tts.is_empty() {
                        return MacroIr::Invalid;
                    }
                    let mut style = String::new();

                    for tt in tts {
                        if let TokenTree::Atom(atom) = tt {
                            match atom {
                                TokenTreeAtom::Word(word) => {
                                    style.push_str(word.to_str(text));
                                    style.push(' ');
                                }
                                TokenTreeAtom::QuotedWord(word) => {
                                    style.push('"');
                                    style.extend(word.chars().flat_map(|c| {
                                        iter::once('\\')
                                            .filter(move |_| matches!(c, '"' | '\'' | '\\'))
                                            .chain(iter::once(c))
                                    }));
                                    style.push_str("\" ");
                                }
                                _ => return MacroIr::Invalid,
                            }
                        } else {
                            return MacroIr::Invalid;
                        }
                    }
                    if style.ends_with(' ') {
                        style.pop();
                    }

                    MacroIr::ListStyle(style)
                } else {
                    MacroIr::Invalid
                }
            }
            _ => MacroIr::Invalid,
        }
    }
}
