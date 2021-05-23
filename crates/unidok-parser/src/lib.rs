#[cfg(test)]
#[macro_use]
mod test_macros;

mod blocks;
mod html;
mod inlines;
mod input;
mod macros;
mod parse;
mod parsing_mode;
mod utils;

use crate::blocks::{Context, ParseBlock};
use crate::input::Input;
use crate::parse::{Parse, ParseInfallible};
use crate::utils::Indents;

use aho_corasick::AhoCorasick;
use once_cell::sync::OnceCell;
use unidok_repr::ast::blocks::BlockAst;
use unidok_repr::ir::blocks::AnnBlock;
use unidok_repr::ir::IrState;
use unidok_repr::{IntoIR, SyntaxSpan, ToSpans};

pub struct Doc<'a> {
    pub blocks: Vec<AnnBlock<'a>>,
    pub state: IrState<'a>,
    pub spans: Vec<SyntaxSpan>,
}

pub fn parse(s: &str, retrieve_spans: bool) -> Doc {
    let mut input = Input::new(s);

    static PATTERNS: OnceCell<AhoCorasick> = OnceCell::new();
    let patterns = PATTERNS.get_or_init(ParseBlock::get_global_patterns);

    let parsed = input.parse(ParseBlock::new_global(patterns)).unwrap();
    debug_assert!(input.is_empty());

    let mut spans = Vec::new();
    if retrieve_spans {
        for p in &parsed {
            if let BlockAst::CodeBlock(c) = p {
                c.to_spans(&mut spans);
            }
        }
    }

    let blocks = parsed.into_ir(s, input.state_mut());
    let state = IrState::new(s, input.into_state());
    Doc { blocks, state, spans }
}
