use crate::parser::{
    error::ParseResult,
    operator::{Comparison, ParseOperator},
    token_stream::MergeSpan,
    ParseContext,
};

use super::{
    bitwise::parse_bitwise, BinaryExpression, Expression, ExpressionError, ExpressionKind,
};

pub fn parse_comparison(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
    let mut left = parse_bitwise(context)?;

    while let Some(operator) = Comparison::parse(context) {
        let right = parse_bitwise(context)?;

        let span = left.span.merge(&operator.span).merge(&right.span);
        let kind = ExpressionKind::Comparison(BinaryExpression::new(operator, left, right));

        left = Expression::new(kind, span);
    }

    Ok(left)
}
