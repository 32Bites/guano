use guano_lexer::Token;

use crate::parser::{
    error::{ParseError, ParseResult},
    Parse, ParseContext,
};

use super::{Expression, ExpressionError};

pub fn parse_list(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
    match &context.stream.read::<1>()[0] {
        Some((token, span)) => match token {
            Token::OpenBracket => {
                let mut values = vec![];

                if let Some(Token::CloseBracket) = context.stream.peek_token::<1>()[0] {
                    context.stream.read::<1>();
                } else {
                    context.stream.reset_peek();
                    loop {
                        let value = Expression::parse(context)?;
                        values.push(value);

                        match &context.stream.read::<1>()[0] {
                            Some((token, span)) => match token {
                                Token::Comma => {}
                                Token::CloseBracket => break,
                                _ => return Err(ParseError::unexpected_token(span.clone())),
                            },
                            None => return Err(ParseError::EndOfFile),
                        }
                    }
                }

                Ok(Expression::List(values))
            }
            _ => Err(ParseError::unexpected_token(span.clone())),
        },
        None => Err(ParseError::EndOfFile),
    }
}
