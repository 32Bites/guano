use crate::parser::{
    error::ParseResult,
    operator::{ParseOperator, Unary},
    token_stream::MergeSpan,
    ParseContext,
};

use super::{access::parse_access, Expression, ExpressionError, ExpressionKind};

pub fn parse_unary(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
    if let Some(operator) = Unary::parse(context) {
        let right = parse_unary(context)?;
        let span = operator.span.merge(&right.span);

        let kind = ExpressionKind::Unary {
            operator,
            right: Box::new(right),
        };

        Ok(Expression::new(kind, span))
    } else {
        parse_access(context)
    }
}
