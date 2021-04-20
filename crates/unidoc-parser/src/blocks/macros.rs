use crate::inlines::macros::{ParseMacroArgs, ParseMacroName};
use crate::utils::{Indents, ParseLineBreak, ParseSpaces};
use crate::{Block, Context, Input, Parse, StrSlice};

/// A block macro
///
/// ### Example
///
/// ````md
/// @SOME_MACRO(args)
/// The macro applies to this paragraph
/// ````
#[derive(Debug, Clone, PartialEq)]
pub struct BlockMacro {
    pub name: StrSlice,
    pub args: Option<StrSlice>,
    pub content: Box<Block>,
}

impl BlockMacro {
    pub fn parser(ind: Indents<'_>) -> ParseBlockMacro<'_> {
        ParseBlockMacro { ind }
    }
}

pub struct ParseBlockMacro<'a> {
    ind: Indents<'a>,
}

impl Parse for ParseBlockMacro<'_> {
    type Output = BlockMacro;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        let ind = self.ind.push_indent(input.parse_i(ParseSpaces));

        input.parse('@')?;
        let name = input.parse(ParseMacroName)?;
        let args = input.parse(ParseMacroArgs);

        input.parse(ParseLineBreak(ind))?;
        let node = input.parse(Block::parser(Context::Global, ind))?;
        let content = Box::new(node);

        input.apply();
        Some(BlockMacro { name, args, content })
    }

    fn can_parse(&self, input: &mut Input) -> bool {
        macro_rules! expect_or_ret {
            ($e:expr) => {
                match $e {
                    Some(e) => e,
                    None => return false,
                }
            };
        }

        let mut input = input.start();

        let ind = self.ind.push_indent(input.parse_i(ParseSpaces));

        expect_or_ret!(input.parse('@'));
        if input.parse(ParseMacroName).is_none() {
            return false;
        }
        input.try_parse(ParseMacroArgs);

        expect_or_ret!(input.parse(ParseLineBreak(ind)));
        input.can_parse(Block::parser(Context::Global, ind))
    }
}
