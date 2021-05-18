mod utils;

use serde::Serialize;
use unidok_repr::SyntaxSpan;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Returns a `CompileResult`.
#[wasm_bindgen]
pub fn compile(input: &str, retrieve_spans: Option<bool>) -> JsValue {
    utils::set_panic_hook();

    let retrieve_spans = retrieve_spans.unwrap_or(false);

    let res = unidok_parser::parse(input, retrieve_spans);
    let contains_math = res.state.contains_math;

    let spans = if retrieve_spans { Some(res.spans.clone()) } else { None };

    let nodes = unidok_to_html::convert(res);
    let text = unidok_to_html::to_string(&nodes);

    serde_wasm_bindgen::to_value(&CompileResult { text, contains_math, spans }).unwrap()
}

#[derive(Serialize)]
pub struct CompileResult {
    pub text: String,
    pub contains_math: bool,
    pub spans: Option<Vec<SyntaxSpan>>,
}

#[wasm_bindgen(typescript_custom_section)]
const I_COMPILE_RESULT: &'static str = r#"
declare interface CompileResult {
    text: string;
    contains_math: boolean;
    spans?: SyntaxSpan[];
}

declare type SyntaxSpan = [SyntaxKind, number, number]

declare enum SyntaxKind {
    InlineFormatting,
    Italic,
    Bold,
    Strikethrough,
    Superscript,
    Subscript,
    InlineCode,

    Heading,
    AtxHeading,
    SetextHeading1,
    SetextHeading2,
    AtxHeadingMarker,
    SetextHeadingMarker,

    Link,
    LinkText,
    LinkRef,
    LinkHref,
    LinkTitle,
    LinkRefDef,

    Image,
    ImageAltText,
    ImageHref,
    ImageTitle,

    Footnote,

    Blockquote,
    BlockquoteMarker,

    List,
    OrderedList,
    UnorderedList,
    ListMarker,

    ThematicBreak,

    CodeBlock,
    CodeFence,
    InfoString,

    Table,
    TableCell,
    TableCellMeta,

    Math,
    MathContent,

    Limiter,

    Comment,

    HtmlTag,
    HtmlTagName,
    HtmlAttrName,
    HtmlAttrValue,

    HtmlDoctype,
    HtmlCdata,
    HtmlComment,
    HtmlEntity,

    Macro,
    MacroName,
    MacroArg,
    MacroKey,
    MacroArgString,
    MacroArgList,
    CurlyBraces,

    Escaped,
}
"#;
