use crate::parser::{
    error::ParseResult,
    operator::{Factor, ParseOperator},
    token_stream::MergeSpan,
    ParseContext,
};

use super::{cast::parse_cast, BinaryExpression, Expression, ExpressionError, ExpressionKind};

pub fn parse_factor(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
    let mut left = parse_cast(context)?;

    while let Some(operator) = Factor::parse(context) {
        let right = parse_cast(context)?;

        let span = left.span.merge(&operator.span).merge(&right.span);
        let kind = ExpressionKind::Factor(BinaryExpression::new(operator, left, right));

        left = Expression::new(kind, span);
    }

    Ok(left)
}
