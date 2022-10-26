use crate::parser::{
    error::ParseResult,
    expression::simplify::Simplify,
    operator::{Comparison, ParseOperator},
    ParseContext,
};

use super::{bitwise::parse_bitwise, BinaryExpression, Expression, ExpressionError};

pub fn parse_comparison(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
    let mut left = parse_bitwise(context)?;

    while let Some(operator) = Comparison::parse(context) {
        let right = parse_bitwise(context)?;

        left = Expression::Comparison(BinaryExpression::new(operator, left, right))
            .simplify_comparison(context.simplified_expressions);
    }

    Ok(left)
}
