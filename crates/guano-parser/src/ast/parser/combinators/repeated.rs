use crate::ast::parser::{error::Error, Parser, ParserContext};

use super::Combinators;

#[derive(Debug, Clone, Copy)]
pub struct Repeated<P> {
    parser: P,
    at_least: Option<u32>,
}

#[inline]
pub fn repeated<'source, P: Parser<'source>>(parser: P) -> Repeated<P> {
    Repeated {
        parser,
        at_least: None,
    }
}

#[inline]
pub fn at_least<'source, P: Parser<'source>>(parser: P, at_least: u32) -> Repeated<P> {
    Repeated {
        parser,
        at_least: Some(at_least),
    }
}

impl<'source, P> Parser<'source> for Repeated<P>
where
    P: Parser<'source, Error = Error<'source>> + Clone,
{
    type Output = Vec<P::Output>;
    type Error = P::Error;

    fn parse(self, context: &mut ParserContext<'source>) -> Result<Self::Output, Self::Error> {
        let mut output;

        if let Some(count) = self.at_least {
            output = self.parser.clone().count(count).parse(context)?;
        } else {
            output = vec![];
        }

        while let Some(item) = self.parser.clone().optional().parse(context)? {
            output.push(item);
        }

        Ok(output)
    }
}
