mod utils;

use serde::Serialize;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(typescript_custom_section)]
const I_COMPILE_RESULT: &'static str = r#"
interface CompileResult {
    text: string;
    contains_math: boolean;
}
"#;

#[wasm_bindgen]
pub fn compile(input: &str) -> JsValue {
    utils::set_panic_hook();

    let res = unidok_parser::parse(input);
    let contains_math = res.state.contains_math;

    let nodes = unidok_to_html::convert(res);
    let text = unidok_to_html::to_string(&nodes);

    serde_wasm_bindgen::to_value(&CompileResult { text, contains_math }).unwrap()
}

#[derive(Serialize)]
pub struct CompileResult {
    pub text: String,
    pub contains_math: bool,
}
