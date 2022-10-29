use guano_lexer::Token;

use crate::parser::{
    error::{ParseError, ParseResult},
    token_stream::MergeSpan,
    Parse, ParseContext,
};

use super::{external::parse_external, Expression, ExpressionError, ExpressionKind};

pub fn parse_access(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
    let mut value = Expression::primary(context)?;

    loop {
        match &context.stream.peek::<1>()[0] {
            Some((token, span)) => match token {
                Token::Period => {
                    context.stream.read::<1>();

                    let accessor = parse_external(context)?;
                    let final_span = value.span.merge(span).merge(&accessor.span);

                    let kind = match accessor.kind {
                        ExpressionKind::FunctionCall(method) => ExpressionKind::MethodCall {
                            value: Box::new(value),
                            method,
                        },
                        ExpressionKind::Variable(property) => ExpressionKind::Property {
                            value: Box::new(value),
                            property,
                        },
                        _ => unreachable!(),
                    };

                    value = Expression::new(kind, final_span);
                }
                Token::OpenBracket => {
                    context.stream.read::<1>();
                    let index_expr = Expression::parse(context)?;
                    let start_span = value.span.merge(span).merge(&index_expr.span);

                    match &context.stream.read::<1>()[0] {
                        Some((token, span)) => match token {
                            Token::CloseBracket => {
                                let final_span = start_span.merge(span);

                                let kind = ExpressionKind::Index {
                                    value: Box::new(value),
                                    index: Box::new(index_expr),
                                };

                                value = Expression::new(kind, final_span);
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
            },

            _ => {
                context.stream.reset_peek();
                break;
            }
        }
    }

    context.stream.reset_peek();

    Ok(value)
}
