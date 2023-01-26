use guano_common::rowan::TextRange;
use guano_syntax::{Child, SyntaxNode};

use crate::parsing::{error::Error, ParseContext, Parser};

use super::errors::CombinatorError;

#[derive(Debug, Clone, Copy)]
pub struct Ast<P> {
    parser: P,
}

pub fn ast<'source, P>(parser: P) -> Ast<P>
where
    P: Parser<'source, Output = Child, Error = Error<'source>>,
{
    Ast { parser }
}

impl<'source, P> Parser<'source> for Ast<P>
where
    P: Parser<'source, Output = Child, Error = Error<'source>>,
{
    type Output = SyntaxNode;
    type Error = Error<'source>;

    /// Will consume input regardless of failure.
    fn parse(self, context: &mut ParseContext<'source>) -> Result<Self::Output, Self::Error> {
        let start_pos = context.position();
        let child = self.parser.parse_ast(context)?;
        let end_pos = context.position();

        if let Some(child) = child {
            Ok(child)
        } else {
            Err(Error::spanned(
                TextRange::new(start_pos, end_pos),
                CombinatorError::Ast,
            ))
        }
    }
}
