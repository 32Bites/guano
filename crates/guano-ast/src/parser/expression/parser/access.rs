use guano_lexer::Token;

use crate::parser::{
    error::{ParseError, ParseResult},
    Parse, ParseContext,
};

use super::{external::parse_external, Expression, ExpressionError};

pub fn parse_access(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
    let mut value = Expression::primary(context)?;

    loop {
        match context.stream.peek_token::<1>()[0] {
            Some(Token::Period) => {
                context.stream.read::<1>();

                value = match parse_external(context)? {
                    Expression::FunctionCall(method) => Expression::MethodCall {
                        value: Box::new(value),
                        method,
                    },
                    Expression::Variable(property) => Expression::Property {
                        value: Box::new(value),
                        property,
                    },
                    _ => unreachable!(),
                };
            }
            Some(Token::OpenBracket) => {
                context.stream.read::<1>();
                let index = Expression::parse(context)?;

                match &context.stream.read::<1>()[0] {
                    Some((token, span)) => match token {
                        Token::CloseBracket => {
                            value = Expression::Index {
                                value: Box::new(value),
                                index: Box::new(index),
                            };
                        }
                        _ => return Err(ParseError::unexpected_token(span.clone())),
                    },
                    None => return Err(ParseError::EndOfFile),
                }
            }
            _ => {
                context.stream.reset_peek();
                break;
            }
        }
    }

    context.stream.reset_peek();

    Ok(value)
}
