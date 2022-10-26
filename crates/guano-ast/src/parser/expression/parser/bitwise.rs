use crate::parser::{
    error::ParseResult,
    expression::simplify::Simplify,
    operator::{
        bitwise::{BitwiseParser, ShiftParser},
        ParseOperator,
    },
    ParseContext,
};

use super::{term::parse_term, BinaryExpression, Expression, ExpressionError};

pub fn parse_bitwise(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
    let mut left = parse_bitshift(context)?;

    while let Some(operator) = BitwiseParser::parse(context) {
        let right = parse_bitshift(context)?;

        left = Expression::Bitwise(BinaryExpression::new(operator, left, right))
            .simplify_bitwise(context.simplified_expressions);
    }

    Ok(left)
}

fn parse_bitshift(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
    let mut left = parse_term(context)?;

    while let Some(operator) = ShiftParser::parse(context) {
        let right = parse_term(context)?;

        left = Expression::Bitwise(BinaryExpression::new(operator, left, right))
            .simplify_bitwise(context.simplified_expressions);
    }

    Ok(left)
}
