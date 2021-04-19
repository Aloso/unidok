#[cfg(test)]
#[macro_use]
mod test_macros;

pub mod containers;
pub mod inlines;
pub mod ir;
pub mod leaves;

mod utils;

mod blocks;

mod input;
mod parse;

use crate::blocks::{Block, Context};
use crate::input::Input;
use crate::ir::{DocIr, IntoIR};
use crate::parse::Parse;
use crate::utils::cond::{UntilChar, WhileChar};
use detached_str as str;

pub fn parse(s: &str) -> DocIr {
    let mut input = Input::new(s);
    let parsed = input.parse(Block::global_parser()).unwrap();
    DocIr { blocks: parsed.into_ir(s) }
}
