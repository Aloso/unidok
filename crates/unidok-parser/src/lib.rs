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
mod state;
mod utils;

use crate::blocks::ParseBlock;
use crate::input::Input;
use crate::parse::{Parse, ParseInfallible};
use crate::state::{Context, ParsingState};
use crate::utils::Indents;

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

    let parsed = input.parse(ParseBlock::new_multi(None, ParsingState::new_global())).unwrap();
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
