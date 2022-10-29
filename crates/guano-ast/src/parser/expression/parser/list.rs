use guano_lexer::Token;

use crate::parser::{
    error::{ParseError, ParseResult},
    token_stream::MergeSpan,
    Parse, ParseContext,
};

use super::{Expression, ExpressionError, ExpressionKind};

pub fn parse_list(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
    match &context.stream.read::<1>()[0] {
        Some((token, span)) => match token {
            Token::OpenBracket => {
                let mut values = vec![];
                let mut final_span = span.clone();

                if let Some((Token::CloseBracket, span)) = &context.stream.peek::<1>()[0] {
                    context.stream.read::<1>();
                    final_span = final_span.merge(span);
                } else {
                    context.stream.reset_peek();
                    loop {
                        let value = Expression::parse(context)?;
                        final_span = final_span.merge(&value.span);
                        values.push(value);

                        match &context.stream.read::<1>()[0] {
                            Some((token, span)) => match token {
                                Token::Comma => {
                                    final_span = final_span.merge(span);
                                }
                                Token::CloseBracket => {
                                    final_span = final_span.merge(span);
                                    break;
                                }
                                _ => return Err(ParseError::unexpected_token(span.clone())),
                            },
                            None => return Err(ParseError::EndOfFile),
                        }
                    }
                }

                let kind = ExpressionKind::List(values);

                Ok(Expression::new(kind, final_span))
            }
            _ => Err(ParseError::unexpected_token(span.clone())),
        },
        None => Err(ParseError::EndOfFile),
    }
}
