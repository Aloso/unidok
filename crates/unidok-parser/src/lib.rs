#[cfg(test)]
#[macro_use]
mod test_macros;

mod blocks;
mod html;
mod inlines;
mod macros;

mod input;
mod parse;
mod parsing_mode;
mod utils;

use crate::blocks::{Context, ParseBlock};
use crate::input::Input;
use crate::parse::{Parse, ParseInfallible};
use crate::utils::Indents;

use unidok_repr::ir::blocks::AnnBlock;
use unidok_repr::ir::IrState;
use unidok_repr::IntoIR;

pub struct DocIr<'a> {
    pub blocks: Vec<AnnBlock<'a>>,
    pub state: IrState<'a>,
}

pub fn parse(s: &str) -> DocIr {
    let mut input = Input::new(s);
    let parsed = input.parse(ParseBlock::new_global()).unwrap();
    debug_assert!(input.is_empty());

    let blocks = parsed.into_ir(s, input.state_mut());
    let state = IrState::new(s, input.into_state());
    DocIr { blocks, state }
}
