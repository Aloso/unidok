use detached_str::StrSlice;

use crate::ast::blocks::BlockAst;
use crate::ast::segments::{BracesAst, SegmentAst};

#[derive(Debug, Clone, PartialEq)]
pub struct BlockMacro {
    pub name: StrSlice,
    pub args: Option<MacroArgs>,
    pub content: BlockMacroContent,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InlineMacroAst {
    pub name: StrSlice,
    pub args: Option<MacroArgs>,
    pub segment: Box<SegmentAst>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockMacroContent {
    Prefixed(Box<BlockAst>),
    Braces(Vec<BlockAst>),
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MacroArgs {
    Raw(StrSlice),
    TokenTrees(Vec<TokenTree>),
}

impl MacroArgs {
    pub fn as_token_trees(&self) -> Option<&[TokenTree]> {
        if let MacroArgs::TokenTrees(t) = self {
            Some(t)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenTree {
    Atom(TokenTreeAtom),
    KV(StrSlice, TokenTreeAtom),
}

impl TokenTree {
    pub fn as_atom(&self) -> Option<&TokenTreeAtom> {
        if let Self::Atom(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenTreeAtom {
    Word(StrSlice),
    QuotedWord(String),
    Tuple(Vec<TokenTree>), // [foo=bar, baz="", quux]
    Braces(BracesAst),
}

impl TokenTreeAtom {
    pub fn as_word(&self) -> Option<StrSlice> {
        if let Self::Word(v) = *self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_quoted_word(&self) -> Option<&str> {
        if let Self::QuotedWord(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_str<'a>(&'a self, text: &'a str) -> Option<&'a str> {
        match self {
            TokenTreeAtom::Word(w) => Some(w.to_str(text)),
            TokenTreeAtom::QuotedWord(w) => Some(w),
            _ => None,
        }
    }
}
