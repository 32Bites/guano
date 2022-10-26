use guano_lexer::Token;

use crate::parser::{
    error::{ParseResult, ToParseResult},
    expression::simplify::Simplify,
    typing::Type,
    Parse, ParseContext,
};

use super::{unary::parse_unary, Expression, ExpressionError};

pub fn parse_cast(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
    let mut left = parse_unary(context)?;

    loop {
        match context.stream.peek_token::<1>()[0] {
            Some(Token::KeyAs) => {
                context.stream.read::<1>();
                let cast_type = Type::parse(context).to_parse_result()?;

                left = Expression::Cast {
                    value: Box::new(left),
                    new_type: cast_type,
                }
                .simplify_cast(context.simplified_expressions);
            }
            _ => {
                context.stream.reset_peek();
                break;
            }
        }
    }

    Ok(left)
}
