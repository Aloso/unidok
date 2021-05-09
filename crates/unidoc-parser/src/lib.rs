#[cfg(test)]
#[macro_use]
mod test_macros;

pub mod blocks;
pub mod html;
pub mod inlines;
pub mod ir;
pub mod macros;
pub mod visitor;

mod collapse_text;
mod input;
mod parse;
mod parser_state;
mod parsing_mode;
mod utils;

use crate::blocks::{Block, Context};
use crate::collapse_text::collapse_text;
use crate::input::Input;
use crate::ir::{DocIr, IntoIR};
use crate::parse::{Parse, ParseInfallible};

pub use detached_str::{Str, StrSlice, StrSliceIndex};

pub fn parse(s: &str) -> DocIr {
    let mut input = Input::new(s);
    let parsed = input.parse(Block::global_parser()).unwrap();
    DocIr { blocks: parsed.into_ir(s, &input) }
}
