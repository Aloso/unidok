use crate::inlines::macros::ParseMacroName;
use crate::macros::MacroArgs;
use crate::utils::{Indents, ParseLineBreak, ParseLineEnd, ParseSpaces, ParseSpacesU8};
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
    pub args: Option<MacroArgs>,
    pub content: BlockMacroContent,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockMacroContent {
    Prefixed(Box<Block>),
    Braces(Vec<Block>),
}

impl BlockMacro {
    pub fn parser(context: Context, ind: Indents<'_>) -> ParseBlockMacro<'_> {
        ParseBlockMacro { context, ind }
    }
}

pub struct ParseBlockMacro<'a> {
    context: Context,
    ind: Indents<'a>,
}

impl Parse for ParseBlockMacro<'_> {
    type Output = BlockMacro;

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        let ind = self.ind.push_indent(input.parse(ParseSpacesU8)?);

        input.parse('@')?;
        let name = input.parse(ParseMacroName)?;
        let name_str = name.to_str(input.text()).to_string();
        let args = input.parse(MacroArgs::parser(&name_str, ind))?;

        let mac = if input.parse(ParseLineBreak(ind)).is_some() {
            let block = Box::new(input.parse(Block::parser(self.context, ind))?);

            BlockMacro { name, args, content: BlockMacroContent::Prefixed(block) }
        } else if input.parse(ParseOpeningBrace(self.ind)).is_some() {
            let blocks = input.parse(Block::multi_parser(Context::BlockBraces, ind))?;
            input.try_parse(ParseClosingBrace(self.ind));

            BlockMacro { name, args, content: BlockMacroContent::Braces(blocks) }
        } else {
            return None;
        };

        input.apply();
        Some(mac)
    }
}

pub(crate) struct ParseOpeningBrace<'a>(pub(crate) Indents<'a>);

impl Parse for ParseOpeningBrace<'_> {
    type Output = ();

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse('{')?;
        input.parse_i(ParseSpaces);
        input.parse(ParseLineEnd)?;
        input.try_parse(ParseLineBreak(self.0));

        input.apply();
        Some(())
    }
}

pub(crate) struct ParseClosingBrace<'a>(pub(crate) Indents<'a>);

impl Parse for ParseClosingBrace<'_> {
    type Output = ();

    fn parse(&self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse_i(ParseSpaces);
        input.parse('}')?;
        input.parse_i(ParseSpaces);
        input.parse(ParseLineEnd)?;
        input.try_parse(ParseLineBreak(self.0));

        input.apply();
        Some(())
    }
}
