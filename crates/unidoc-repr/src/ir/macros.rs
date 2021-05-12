use super::segments::BracesIr;

#[derive(Debug, Clone, PartialEq)]
pub enum MacroArgsIr<'a> {
    Raw(&'a str),
    TokenTrees(Vec<TokenTreeIr<'a>>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenTreeIr<'a> {
    Atom(TokenTreeAtomIr<'a>),
    KV(&'a str, TokenTreeAtomIr<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenTreeAtomIr<'a> {
    Word(&'a str),
    QuotedWord(String),
    Tuple(Vec<TokenTreeIr<'a>>),
    Braces(BracesIr<'a>),
}
