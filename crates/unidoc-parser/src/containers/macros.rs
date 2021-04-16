use crate::inlines::macros::{ParseMacroArgs, ParseMacroName};
use crate::str::StrSlice;
use crate::utils::{Indents, ParseLineBreak, ParseSpaces};
use crate::{Input, Node, NodeCtx, Parse};

#[derive(Debug, Clone, PartialEq)]
pub struct BlockMacro {
    pub name: StrSlice,
    pub args: Option<StrSlice>,
    pub content: Box<Node>,
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

        let ind = self.ind.push_indent(input.parse(ParseSpaces)?);

        input.parse('@')?;
        let name = input.parse(ParseMacroName)?;
        let args = input.parse(ParseMacroArgs);

        input.parse(ParseLineBreak(ind))?;
        let node = input.parse(Node::parser(NodeCtx::ContainerOrGlobal, ind))?;
        let content = Box::new(node);

        input.apply();
        Some(BlockMacro { name, args, content })
    }
}
