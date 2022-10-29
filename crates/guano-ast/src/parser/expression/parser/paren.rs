use guano_lexer::Token;

use crate::parser::{
    error::{ParseError, ParseResult},
    token_stream::MergeSpan,
    Parse, ParseContext,
};

use super::{Expression, ExpressionError, ExpressionKind};

pub fn parse_paren(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
    match &context.stream.read::<1>()[0] {
        Some((token, span)) => match token {
            Token::OpenParen => {
                let mut values = vec![];
                let mut is_tuple = false;
                let mut final_span = span.clone();

                loop {
                    if let Some((Token::CloseParen, span)) = &context.stream.peek::<1>()[0] {
                        final_span = final_span.merge(span);
                        context.stream.read::<1>();
                        if values.len() == 0 {
                            is_tuple = true;
                        }
                        break;
                    }
                    context.stream.reset_peek();

                    if is_tuple || values.len() == 0 {
                        let expr = Expression::parse(context)?;
                        final_span = final_span.merge(&expr.span);

                        values.push(expr);
                    } else {
                        return Err(ParseError::unexpected_token(
                            context.stream.peek_span::<1>()[0].clone().flatten(),
                        ));
                    }

                    match &context.stream.peek::<1>()[0] {
                        Some((token, span)) => match token {
                            Token::Comma => {
                                final_span = final_span.merge(&span);
                                context.stream.read::<1>();
                                is_tuple = true;
                            }
                            Token::CloseParen => context.stream.reset_peek(),
                            _ => return Err(ParseError::unexpected_token(span.clone())),
                        },
                        None => return Err(ParseError::EndOfFile),
                    }
                }

                let kind = if is_tuple {
                    ExpressionKind::Tuple(values)
                } else if values.len() == 1 {
                    ExpressionKind::Group(Box::new(values.remove(0)))
                } else {
                    unreachable!()
                };

                Ok(Expression::new(kind, final_span))
            }
            _ => Err(ParseError::unexpected_token(span.clone())),
        },
        None => Err(ParseError::EndOfFile),
    }
}
