use guano_lexer::Token;

use crate::parser::{
    error::{ParseError, ParseResult},
    expression::simplify::Simplify,
    Parse, ParseContext,
};

use super::{Expression, ExpressionError};

pub fn parse_paren(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
    match &context.stream.read::<1>()[0] {
        Some((token, span)) => match token {
            Token::OpenParen => {
                let mut values = vec![];
                let mut is_tuple = false;

                loop {
                    if let Some(Token::CloseParen) = context.stream.peek_token::<1>()[0] {
                        context.stream.read::<1>();
                        if values.len() == 0 {
                            is_tuple = true;
                        }
                        break;
                    }
                    context.stream.reset_peek();

                    if is_tuple || values.len() == 0 {
                        values.push(Expression::parse(context)?);
                    } else {
                        return Err(ParseError::unexpected_token(
                            context.stream.peek_span::<1>()[0].clone().flatten(),
                        ));
                    }

                    match &context.stream.peek::<1>()[0] {
                        Some((token, span)) => match token {
                            Token::Comma => {
                                context.stream.read::<1>();
                                is_tuple = true;
                            }
                            Token::CloseParen => context.stream.reset_peek(),
                            _ => return Err(ParseError::unexpected_token(span.clone())),
                        },
                        None => return Err(ParseError::EndOfFile),
                    }
                }

                if is_tuple {
                    Ok(Expression::Tuple(values))
                } else if values.len() == 1 {
                    Ok(Expression::Group(Box::new(values.remove(0)))
                        .simplify_group(context.simplified_expressions))
                } else {
                    unreachable!()
                }
            }
            _ => Err(ParseError::unexpected_token(span.clone())),
        },
        None => Err(ParseError::EndOfFile),
    }
}
