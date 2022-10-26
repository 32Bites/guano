use crate::parser::{
    error::ParseResult,
    expression::simplify::Simplify,
    operator::{Factor, ParseOperator},
    ParseContext,
};

use super::{cast::parse_cast, BinaryExpression, Expression, ExpressionError};

pub fn parse_factor(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
    let mut left = parse_cast(context)?;

    while let Some(operator) = Factor::parse(context) {
        let right = parse_cast(context)?;

        left = Expression::Factor(BinaryExpression::new(operator, left, right))
            .simplify_factor(context.simplified_expressions);
    }

    Ok(left)
}
