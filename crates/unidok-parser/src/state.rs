use aho_corasick::AhoCorasick;
use once_cell::sync::OnceCell;
use unidok_repr::ast::html::ElemName;

use crate::inlines::segments;
use crate::utils::Indents;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum Context {
    InlineBraces,
    BlockBraces,
    Table,
    LinkOrImg,
    Code(u8),
    CodeBlock,
    Heading,
    InlineHtml(ElemName),
    BlockHtml(ElemName),
    Global,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ParsingState<'a> {
    indentation: Indents<'a>,
    context: Context,
    special_chars: &'a AhoCorasick,
}

impl<'a> ParsingState<'a> {
    pub(crate) fn new(
        indentation: Indents<'a>,
        context: Context,
        special_chars: &'a AhoCorasick,
    ) -> Self {
        ParsingState { indentation, context, special_chars }
    }

    pub(crate) fn new_global() -> Self {
        static PATTERNS: OnceCell<AhoCorasick> = OnceCell::new();
        let special_chars = PATTERNS.get_or_init(segments::get_global_patterns);

        ParsingState { indentation: Indents::new(), context: Context::Global, special_chars }
    }

    pub(crate) fn ind(&self) -> Indents {
        self.indentation
    }

    pub(crate) fn context(&self) -> Context {
        self.context
    }

    pub(crate) fn special_chars(&self) -> &AhoCorasick {
        self.special_chars
    }
}
