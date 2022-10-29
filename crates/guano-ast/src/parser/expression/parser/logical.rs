use crate::parser::{
    error::ParseResult,
    operator::{Logical, ParseOperator},
    token_stream::MergeSpan,
    ParseContext,
};

use super::{
    comparison::parse_comparison, BinaryExpression, Expression, ExpressionError, ExpressionKind,
};

pub fn parse_logical(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
    let mut left = parse_comparison(context)?;

    while let Some(operator) = Logical::parse(context) {
        let start = left.span.clone();
        let right = parse_comparison(context)?;
        let span = start.merge(&operator.span).merge(&right.span);

        let kind = ExpressionKind::Logical(BinaryExpression::new(operator, left, right));

        left = Expression::new(kind, span);
    }

    Ok(left)
}
