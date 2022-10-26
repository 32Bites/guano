use guano_lexer::Token;

use crate::parser::{
    error::{ParseError, ParseResult, ToParseResult},
    identifier::Identifier,
    Parse, ParseContext,
};

use super::{Expression, ExpressionError, FunctionCall};

pub fn parse_external(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
    let identifier = Identifier::parse(context).to_parse_result()?;
    if let Some(Token::OpenParen) = context.stream.peek_token::<1>()[0] {
        context.stream.read::<1>();

        let mut arguments = vec![];

        if let Some(Token::CloseParen) = context.stream.peek_token::<1>()[0] {
            context.stream.read::<1>();
        } else {
            context.stream.reset_peek();
            loop {
                let argument = Expression::parse(context).to_parse_result()?;
                arguments.push(argument);

                match &context.stream.read::<1>()[0] {
                    Some((token, span)) => match token {
                        Token::Comma => {}
                        Token::CloseParen => break,
                        _ => return Err(ParseError::unexpected_token(span.clone())),
                    },
                    None => return Err(ParseError::EndOfFile),
                }
            }
        }

        Ok(Expression::FunctionCall(FunctionCall {
            identifier,
            arguments,
        }))
    } else {
        context.stream.reset_peek();
        Ok(Expression::Variable(identifier))
    }
}
