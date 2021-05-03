mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn compile(input: &str) -> String {
    utils::set_panic_hook();

    let res = unidoc_parser::parse(input);
    let nodes = unidoc_to_html::convert(res);
    unidoc_to_html::to_html(&nodes)
}
