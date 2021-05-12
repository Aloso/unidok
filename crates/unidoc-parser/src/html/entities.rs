use unidoc_repr::ast::html::HtmlEntity;

use crate::{Input, Parse};

pub(crate) struct ParseHtmlEntity;

impl Parse for ParseHtmlEntity {
    type Output = HtmlEntity;

    fn parse(&mut self, input: &mut Input) -> Option<Self::Output> {
        let mut input = input.start();

        input.parse('&')?;
        let rest = input.rest().as_bytes();
        let rest = if rest.len() > 32 { &rest[..32] } else { rest };

        let mut idx = 0;
        for b in rest.iter().copied() {
            if b == b';' {
                idx += 1;
                break;
            } else if b.is_ascii_alphabetic() {
                idx += 1;
            } else {
                break;
            }
        }

        let entity = &rest[..idx];
        let entity = HtmlEntity::from_bytes(entity)?;
        input.bump(entity.0.len());

        input.apply();
        Some(entity)
    }
}
