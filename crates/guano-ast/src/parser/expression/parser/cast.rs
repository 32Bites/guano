use guano_lexer::Token;

use crate::parser::{
    error::{ParseResult, ToParseResult},
    token_stream::MergeSpan,
    typing::Type,
    Parse, ParseContext,
};

use super::{unary::parse_unary, Expression, ExpressionError, ExpressionKind};

pub fn parse_cast(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
    let mut left = parse_unary(context)?;

    loop {
        match &context.stream.peek::<1>()[0] {
            Some((Token::KeyAs, span)) => {
                context.stream.read::<1>();
                let cast_type = Type::parse(context).to_parse_result()?;

                let span = left.span.merge(span).merge(&cast_type.span);
                let kind = ExpressionKind::Cast {
                    value: Box::new(left),
                    new_type: cast_type,
                };

                left = Expression::new(kind, span);
            }
            _ => {
                context.stream.reset_peek();
                break;
            }
        }
    }

    Ok(left)
}
