use crate::parser::{
    error::ParseResult,
    operator::{ParseOperator, Term},
    token_stream::MergeSpan,
    ParseContext,
};

use super::{factor::parse_factor, BinaryExpression, Expression, ExpressionError, ExpressionKind};

pub fn parse_term(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
    let mut left = parse_factor(context)?;

    while let Some(operator) = Term::parse(context) {
        let right = parse_factor(context)?;

        let span = left.span.merge(&operator.span).merge(&right.span);
        let kind = ExpressionKind::Term(BinaryExpression::new(operator, left, right));

        left = Expression::new(kind, span);
    }

    Ok(left)
}
