#[cfg(test)]
#[macro_use]
mod test_macros;

mod accumulate;
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
use crate::parse::{Parse, ParseInfallible};
use crate::state::{Context, ParsingState};
use crate::utils::Indents;

use unidok_repr::ast::AstData;
use unidok_repr::config::Config;
use unidok_repr::ir::blocks::AnnBlock;
use unidok_repr::ir::IrState;
use unidok_repr::{IntoIR, SyntaxSpan, ToSpans};

pub use crate::input::Input;

pub struct Doc<'a> {
    pub blocks: Vec<AnnBlock<'a>>,
    pub state: IrState<'a>,
    pub spans: Vec<SyntaxSpan>,
}

pub fn parse(input: &mut Input, config: Config) -> Doc<'_> {
    let parsed = input.parse(ParseBlock::new_multi(None, ParsingState::new_global())).unwrap();
    assert!(input.is_empty());

    let mut spans = Vec::new();
    if config.retrieve_spans {
        for p in &parsed {
            p.to_spans(&mut spans);
        }
    }

    let mut data = AstData::new(config);
    accumulate::accumulate_block_data(&parsed, &mut data, false, &input.text);

    let blocks = parsed.into_ir(&input.text, &mut data);
    let state = IrState::new(&input.text, data);
    Doc { blocks, state, spans }
}
