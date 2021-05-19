use std::{iter, mem};

use detached_str::StrSlice;

use crate::ast::macros::*;
use crate::ast::AstState;
use crate::config::HeadingAnchor;
use crate::ir::blocks::{AnnBlock, Block};
use crate::ir::html::HtmlNode;
use crate::ir::macros::{Attr, AttrValue, Footnote, Macro};
use crate::ir::segments::Segment;
use crate::IntoIR;

impl<'a> IntoIR<'a> for BlockMacro {
    type IR = AnnBlock<'a>;

    fn into_ir(self, text: &'a str, state: &mut AstState) -> Self::IR {
        let mut block = self.content.into_ir(text, state);
        let r#macro = MacroAst { name: self.name, args: self.args }.into_ir(text, state);

        if r#macro.is_for_list() {
            if let AnnBlock { block: Block::List(list), .. } = &mut block {
                list.macros.push(r#macro);
            }
        } else {
            block.macros.push(r#macro);
        }

        block
    }
}

impl<'a> IntoIR<'a> for InlineMacroAst {
    type IR = Segment<'a>;

    fn into_ir(self, text: &'a str, state: &mut AstState) -> Self::IR {
        let mut segment = (*self.segment).into_ir(text, state);
        let r#macro = MacroAst { name: self.name, args: self.args };
        match &mut segment {
            Segment::Braces(b) => b.macros.push(r#macro.into_ir(text, state)),
            Segment::Math(b) => b.macros.push(r#macro.into_ir(text, state)),
            Segment::Link(b) => b.macros.push(r#macro.into_ir(text, state)),
            Segment::Image(b) => b.macros.push(r#macro.into_ir(text, state)),
            Segment::Code(b) => b.macros.push(r#macro.into_ir(text, state)),
            Segment::InlineHtml(HtmlNode::Element(b)) => {
                b.macros.push(r#macro.into_ir(text, state))
            }

            _ => {}
        }
        segment
    }
}

impl<'a> IntoIR<'a> for BlockMacroContent {
    type IR = AnnBlock<'a>;

    fn into_ir(self, text: &'a str, state: &mut AstState) -> Self::IR {
        match self {
            BlockMacroContent::Prefixed(p) => (*p).into_ir(text, state),
            BlockMacroContent::Braces(b) => {
                AnnBlock { macros: vec![], block: Block::Braces(b.into_ir(text, state)) }
            }
        }
    }
}

struct MacroAst {
    name: StrSlice,
    args: Option<MacroArgs>,
}

impl<'a> IntoIR<'a> for MacroAst {
    type IR = Macro<'a>;

    fn into_ir(self, text: &'a str, state: &mut AstState) -> Self::IR {
        match self.name.to_str(text) {
            "" => {
                if let Some(MacroArgs::TokenTrees(tts)) = self.args {
                    if tts.is_empty() {
                        return Macro::Invalid;
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
                            _ => return Macro::Invalid,
                        }
                    }

                    Macro::HtmlAttrs(result)
                } else {
                    Macro::Invalid
                }
            }
            "TOC" => {
                if self.args.is_none() {
                    Macro::Toc
                } else {
                    Macro::Invalid
                }
            }
            "NOTOC" => {
                if self.args.is_none() {
                    Macro::NoToc
                } else {
                    Macro::Invalid
                }
            }
            "NOTXT" => {
                if self.args.is_none() {
                    Macro::NoText
                } else {
                    Macro::Invalid
                }
            }
            "LOOSE" => {
                if self.args.is_none() {
                    Macro::Loose
                } else {
                    Macro::Invalid
                }
            }
            "BULLET" => {
                if let Some(MacroArgs::TokenTrees(tts)) = self.args {
                    if tts.is_empty() {
                        return Macro::Invalid;
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
                                _ => return Macro::Invalid,
                            }
                        } else {
                            return Macro::Invalid;
                        }
                    }
                    if style.ends_with(' ') {
                        style.pop();
                    }

                    Macro::ListStyle(style)
                } else {
                    Macro::Invalid
                }
            }
            "MATH_SCRIPT" => {
                if self.args.is_none() {
                    Macro::MathScript
                } else {
                    Macro::Invalid
                }
            }
            "BLANK" => {
                if self.args.is_none() {
                    Macro::Blank
                } else {
                    Macro::Invalid
                }
            }
            "FOOTNOTES" => {
                if self.args.is_none() {
                    if state.footnotes.is_empty() {
                        Macro::Footnotes(vec![])
                    } else {
                        let links = mem::take(&mut state.footnotes);
                        let footnotes = links
                            .into_iter()
                            .flat_map(|link| {
                                let num = state.next_footnote_def;
                                state.next_footnote_def += 1;

                                link.text.map(|t| Footnote { num, text: t.into_ir(text, state) })
                            })
                            .collect();
                        Macro::Footnotes(footnotes)
                    }
                } else {
                    Macro::Invalid
                }
            }
            "CONFIG" => {
                if let Some(MacroArgs::TokenTrees(args)) = self.args {
                    for arg in args {
                        if let TokenTree::KV(key, value) = arg {
                            let key = key.to_str(text);
                            if key == "heading_anchor" {
                                match value.as_str(text) {
                                    Some("start" | "before") => {
                                        state.config.heading_anchor = HeadingAnchor::Start
                                    }
                                    Some("end" | "after") => {
                                        state.config.heading_anchor = HeadingAnchor::End
                                    }
                                    Some("none" | "no" | "false") => {
                                        state.config.heading_anchor = HeadingAnchor::None
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    Macro::Config
                } else {
                    Macro::Invalid
                }
            }
            _ => Macro::Invalid,
        }
    }
}
