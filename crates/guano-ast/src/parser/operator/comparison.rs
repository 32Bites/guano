use guano_lexer::Token;
use serde::{Serialize, Deserialize};

use crate::parser::ParseContext;

use super::{ParseOperator, Operator};

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
    fn parse(context: &mut ParseContext) -> Option<Self> {
        let operator = match context.stream.peek_token::<2>() {
            [Some(Token::GreaterThan), n] if !matches!(n, Some(Token::GreaterThan)) => match n {
                Some(Token::Equals) => {
                    context.stream.read::<2>();
                    ComparisonOperator::GreaterThanEquals
                }
                _ => {
                    context.stream.read::<1>();
                    ComparisonOperator::GreaterThan
                }
            },
            [Some(Token::LessThan), n] if !matches!(n, Some(Token::LessThan)) => match n {
                Some(Token::Equals) => {
                    context.stream.read::<2>();
                    ComparisonOperator::LessThanEquals
                }
                _ => {
                    context.stream.read::<1>();
                    ComparisonOperator::LessThan
                }
            },
            [Some(Token::Equals), Some(Token::Equals)] => {
                context.stream.read::<2>();
                ComparisonOperator::Equals
            }
            [Some(Token::Exclamation), Some(Token::Equals)] => {
                context.stream.read::<2>();
                ComparisonOperator::NotEqual
            }
            _ => {
                context.stream.reset_peek();
                return None;
            }
        };

        Some(operator)
    }
}
