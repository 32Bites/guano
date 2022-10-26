use crate::parser::{
    error::ParseResult,
    expression::simplify::Simplify,
    operator::{ParseOperator, Unary},
    ParseContext,
};

use super::{access::parse_access, Expression, ExpressionError};

pub fn parse_unary(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
    if let Some(operator) = Unary::parse(context) {
        let right = Box::new(parse_unary(context)?);

        Ok(Expression::Unary { operator, right }.simplify_unary(context.simplified_expressions))
    } else {
        parse_access(context)
    }
}
