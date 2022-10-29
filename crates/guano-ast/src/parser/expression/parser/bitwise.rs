use crate::parser::{
    error::ParseResult,
    operator::{
        bitwise::{BitwiseParser, ShiftParser},
        ParseOperator,
    },
    token_stream::MergeSpan,
    ParseContext,
};

use super::{term::parse_term, BinaryExpression, Expression, ExpressionError, ExpressionKind};

pub fn parse_bitwise(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
    let mut left = parse_bitshift(context)?;

    while let Some(operator) = BitwiseParser::parse(context) {
        let right = parse_bitshift(context)?;

        let span = left.span.merge(&operator.span).merge(&right.span);
        let kind = ExpressionKind::Bitwise(BinaryExpression::new(operator, left, right));

        left = Expression::new(kind, span);
    }

    Ok(left)
}

fn parse_bitshift(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
    let mut left = parse_term(context)?;

    while let Some(operator) = ShiftParser::parse(context) {
        let right = parse_term(context)?;

        let span = left.span.merge(&operator.span).merge(&right.span);
        let kind = ExpressionKind::Bitwise(BinaryExpression::new(operator, left, right));

        left = Expression::new(kind, span);
    }

    Ok(left)
}
