use unidoc_repr::ast::macros::{TokenTree, TokenTreeAtom};

use crate::inlines::braces::ParseBraces;
use crate::utils::{Indents, ParseSpaces, ParseWsNoBlankLinkes, QuotedStringWithEscapes};
use crate::{Input, Parse};

#[derive(Clone, Copy)]
pub(crate) struct ParseTokenTree<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseTokenTree<'_> {
    type Output = TokenTree;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        match input.peek_char() {
            Some('[' | '{' | '"' | '\'') => {
                let atom = input.parse(ParseTokenTreeAtom { ind: self.ind })?;
                input.apply();
                Some(TokenTree::Atom(atom))
            }
            Some(']' | '}' | ')') => None,
            Some(_) => {
                let rest = input.rest();
                let idx =
                    rest.find(|c| matches!(c, '=' | ' ' | '\t' | '\n' | '\r' | ')' | ']' | '}'));
                if let Some(idx) = idx {
                    let remaining = rest[idx..].trim_start_matches(|c| matches!(c, ' ' | '\t'));
                    if remaining.starts_with('=') {
                        let key = input.bump(idx);
                        input.parse_i(ParseSpaces);
                        input.parse('=').unwrap();
                        input.parse_i(ParseSpaces);
                        let value = input.parse(ParseTokenTreeAtom { ind: self.ind })?;
                        input.apply();
                        return Some(TokenTree::KV(key, value));
                    }
                }

                let atom = input.parse(ParseTokenTreeAtom { ind: self.ind })?;
                input.apply();
                Some(TokenTree::Atom(atom))
            }
            None => None,
        }
    }
}

pub(crate) struct ParseTokenTreeAtom<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseTokenTreeAtom<'_> {
    type Output = TokenTreeAtom;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        match input.peek_char() {
            Some('[') => {
                input.bump(1);
                let tuple = input.parse(ParseTokenTrees { ind: self.ind })?;
                input.parse(']')?;
                input.apply();
                Some(TokenTreeAtom::Tuple(tuple))
            }
            Some('{') => {
                input.bump(1);
                let braces = input.parse(ParseBraces { ind: self.ind })?;
                input.parse('}')?;
                input.apply();
                Some(TokenTreeAtom::Braces(braces))
            }
            Some('"' | '\'') => {
                let content = input.parse(QuotedStringWithEscapes(self.ind))?;
                input.apply();
                Some(TokenTreeAtom::QuotedWord(content))
            }
            Some(_) => {
                let rest = input.rest();
                let idx = rest.find(|c| matches!(c, ' ' | '\t' | '\n' | '\r' | ')' | ']' | '}'))?;
                let word = input.bump(idx);
                input.apply();
                Some(TokenTreeAtom::Word(word))
            }
            None => None,
        }
    }
}

pub(crate) struct ParseTokenTrees<'a> {
    pub ind: Indents<'a>,
}

impl Parse for ParseTokenTrees<'_> {
    type Output = Vec<TokenTree>;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let parser = ParseTokenTree { ind: self.ind };
        let mut token_trees = Vec::new();

        input.parse(ParseWsNoBlankLinkes(self.ind))?;
        if matches!(input.peek_char(), Some('\r' | '\n') | None) {
            return Some(token_trees);
        }

        while let Some(tt) = input.parse(parser) {
            token_trees.push(tt);
            input.parse(ParseWsNoBlankLinkes(self.ind))?;
            if matches!(input.peek_char(), Some('\r' | '\n') | None) {
                return Some(token_trees);
            }
        }

        Some(token_trees)
    }
}
