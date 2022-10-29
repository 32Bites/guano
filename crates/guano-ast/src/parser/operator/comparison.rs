use guano_lexer::Token;
use serde::{Deserialize, Serialize};

use crate::parser::{
    token_stream::{Spanned, ToSpanned},
    ParseContext,
};

use super::{Operator, ParseOperator};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonOperator {
    GreaterThan,
    GreaterThanEquals,
    LessThan,
    LessThanEquals,
    Equals,
    NotEqual,
}

impl std::fmt::Display for ComparisonOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.code())
    }
}

impl Operator for ComparisonOperator {
    type Str = &'static str;

    fn name(&self) -> Self::Str {
        match self {
            ComparisonOperator::GreaterThan => "greater than",
            ComparisonOperator::GreaterThanEquals => "greater than equals",
            ComparisonOperator::LessThan => "less than",
            ComparisonOperator::LessThanEquals => "less than equals",
            ComparisonOperator::Equals => "equals",
            ComparisonOperator::NotEqual => "not equal",
        }
    }

    fn code(&self) -> Self::Str {
        match self {
            ComparisonOperator::GreaterThan => ">",
            ComparisonOperator::GreaterThanEquals => ">=",
            ComparisonOperator::LessThan => "<",
            ComparisonOperator::LessThanEquals => "<=",
            ComparisonOperator::Equals => "==",
            ComparisonOperator::NotEqual => "!=",
        }
    }
}

impl ParseOperator for ComparisonOperator {
    fn parse(context: &mut ParseContext) -> Option<Spanned<Self>> {
        let (operator, span) = match context.stream.peek_token::<2>() {
            [Some(Token::GreaterThan), n] if !matches!(n, Some(Token::GreaterThan)) => match n {
                Some(Token::Equals) => (
                    ComparisonOperator::GreaterThanEquals,
                    context.stream.read_span_combined::<2>()?,
                ),
                _ => (
                    ComparisonOperator::GreaterThan,
                    context.stream.read_span::<1>()[0].clone()?,
                ),
            },
            [Some(Token::LessThan), n] if !matches!(n, Some(Token::LessThan)) => match n {
                Some(Token::Equals) => (
                    ComparisonOperator::LessThanEquals,
                    context.stream.read_span_combined::<2>()?,
                ),
                _ => (
                    ComparisonOperator::LessThan,
                    context.stream.read_span::<1>()[0].clone()?,
                ),
            },
            [Some(Token::Equals), Some(Token::Equals)] => (
                ComparisonOperator::Equals,
                context.stream.read_span_combined::<2>()?,
            ),
            [Some(Token::Exclamation), Some(Token::Equals)] => (
                ComparisonOperator::NotEqual,
                context.stream.read_span_combined::<2>()?,
            ),
            _ => {
                context.stream.reset_peek();
                return None;
            }
        };

        Some(operator.to_spanned(span))
    }
}
