use crate::parser::{
    error::ParseResult,
    expression::simplify::Simplify,
    operator::{Logical, ParseOperator},
    ParseContext,
};

use super::{comparison::parse_comparison, BinaryExpression, Expression, ExpressionError};

pub fn parse_logical(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
    let mut left = parse_comparison(context)?;

    while let Some(operator) = Logical::parse(context) {
        let right = parse_comparison(context)?;

        left = Expression::Logical(BinaryExpression::new(operator, left, right))
            .simplify_logical(context.simplified_expressions);
    }

    Ok(left)
}
