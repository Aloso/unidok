#[cfg(test)]
#[macro_use]
mod test_macros;

pub mod blocks;
pub mod html;
pub mod inlines;
pub mod macros;

mod input;
mod parse;
mod parsing_mode;
mod utils;

use crate::blocks::{Context, ParseBlock};
use crate::input::Input;
use crate::parse::{Parse, ParseInfallible};
use crate::utils::Indents;

pub use detached_str::{Str, StrSlice, StrSliceIndex};
use unidoc_repr::ir::blocks::AnnBlockIr;
use unidoc_repr::ir::IrState;
use unidoc_repr::IntoIR;

pub struct DocIr<'a> {
    pub blocks: Vec<AnnBlockIr<'a>>,
    pub state: IrState<'a>,
}

pub fn parse(s: &str) -> DocIr {
    let mut input = Input::new(s);
    let parsed = input.parse(ParseBlock::new_global()).unwrap();
    debug_assert!(input.is_empty());

    let blocks = parsed.into_ir(s, input.state());
    let state = IrState::new(s, input.into_state());
    DocIr { blocks, state }
}
