use detached_str::StrSlice;

use crate::ast::blocks::Block;
use crate::ast::segments::{Braces, Segment};

#[derive(Debug, Clone, PartialEq)]
pub struct BlockMacro {
    pub name: StrSlice,
    pub args: Option<MacroArgs>,
    pub content: BlockMacroContent,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InlineMacro {
    pub name: StrSlice,
    pub args: Option<MacroArgs>,
    pub segment: Box<Segment>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockMacroContent {
    Prefixed(Box<Block>),
    Braces(Vec<Block>),
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
    Braces(Braces),
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
}
