use crate::parser::{
    error::ParseResult,
    expression::simplify::Simplify,
    operator::{ParseOperator, Term},
    ParseContext,
};

use super::{factor::parse_factor, BinaryExpression, Expression, ExpressionError};

pub fn parse_term(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
    let mut left = parse_factor(context)?;

    while let Some(operator) = Term::parse(context) {
        let right = parse_factor(context)?;

        left = Expression::Term(BinaryExpression::new(operator, left, right))
            .simplify_term(context.simplified_expressions);
    }

    Ok(left)
}
