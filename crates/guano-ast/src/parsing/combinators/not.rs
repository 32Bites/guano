use crate::parsing::{error::Error, ParseContext, Parser};

use super::{errors::CombinatorError, Combinators};

#[derive(Debug, Clone, Copy)]
pub struct Not<P> {
    parser: P,
}

#[inline]
pub fn not<'source, P: Parser<'source>>(parser: P) -> Not<P> {
    Not { parser }
}

impl<'source, P> Parser<'source> for Not<P>
where
    P: Parser<'source>,
{
    type Output = ();
    type Error = Error<'source>;

    #[inline]
    fn parse(self, context: &mut ParseContext<'source>) -> Result<Self::Output, Self::Error> {
        let mut temp_context = context.clone();
        let name = self.parser.name();

        match self.parser.spanned().parse(&mut temp_context) {
            Err(_) => Ok(()),
            Ok((_, span)) => {
                let kind = CombinatorError::Not(name);

                Err(Error::spanned(span, kind))
            }
        }
    }
}
