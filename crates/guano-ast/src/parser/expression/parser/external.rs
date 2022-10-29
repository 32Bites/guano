use guano_lexer::Token;

use crate::parser::{
    error::{ParseError, ParseResult, ToParseResult},
    identifier::Identifier,
    token_stream::MergeSpan,
    Parse, ParseContext,
};

use super::{Expression, ExpressionError, ExpressionKind, FunctionCall};

pub fn parse_external(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
    let identifier = Identifier::parse(context).to_parse_result()?;

    if let Some((Token::OpenParen, span)) = &context.stream.peek::<1>()[0] {
        context.stream.read::<1>();

        let mut final_span = identifier.span.merge(span);
        let mut arguments = vec![];

        if let Some((Token::CloseParen, span)) = &context.stream.peek::<1>()[0] {
            final_span = final_span.merge(span);
            context.stream.read::<1>();
        } else {
            context.stream.reset_peek();
            loop {
                let argument = Expression::parse(context).to_parse_result()?;
                final_span = final_span.merge(&argument.span);

                arguments.push(argument);

                match &context.stream.read::<1>()[0] {
                    Some((token, span)) => match token {
                        Token::Comma => {
                            final_span = final_span.merge(span);
                        }
                        Token::CloseParen => {
                            final_span = final_span.merge(span);
                            break;
                        }
                        _ => return Err(ParseError::unexpected_token(span.clone())),
                    },
                    None => return Err(ParseError::EndOfFile),
                }
            }
        }

        let kind = ExpressionKind::FunctionCall(FunctionCall {
            identifier,
            arguments,
        });

        Ok(Expression::new(kind, final_span))
    } else {
        context.stream.reset_peek();

        let span = identifier.span.clone();
        let kind = ExpressionKind::Variable(identifier);

        Ok(Expression::new(kind, span))
    }
}
