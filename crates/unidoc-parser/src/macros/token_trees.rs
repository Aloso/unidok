use crate::inlines::Braces;
use crate::input::Input;
use crate::utils::{Indents, ParseSpaces, ParseWs, QuotedStringWithEscapes};
use crate::{Parse, ParseInfallible, StrSlice};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenTree {
    Atom(TokenTreeAtom),
    KV(StrSlice, TokenTreeAtom),
}

impl TokenTree {
    pub(crate) fn parser(ind: Indents<'_>) -> ParseTokenTree<'_> {
        ParseTokenTree { ind }
    }

    pub(crate) fn multi_parser(ind: Indents<'_>) -> ParseTokenTrees<'_> {
        ParseTokenTrees { ind }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenTreeAtom {
    Word(StrSlice),
    QuotedWord(String),
    Tuple(Vec<TokenTree>), // [foo=bar, baz="", quux]
    Braces(Braces),
}

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
                let tuple = input.parse_i(TokenTree::multi_parser(self.ind));
                input.parse(']')?;
                input.apply();
                Some(TokenTreeAtom::Tuple(tuple))
            }
            Some('{') => {
                input.bump(1);
                let braces = input.parse(Braces::parser(self.ind))?;
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
    ind: Indents<'a>,
}

impl ParseInfallible for ParseTokenTrees<'_> {
    type Output = Vec<TokenTree>;

    fn parse_infallible(&self, input: &mut Input) -> Self::Output {
        let parser = TokenTree::parser(self.ind);
        let mut token_trees = Vec::new();

        input.parse_i(ParseWs(self.ind));

        while let Some(tt) = input.parse(parser) {
            token_trees.push(tt);
            input.parse_i(ParseWs(self.ind));
        }

        token_trees
    }
}
