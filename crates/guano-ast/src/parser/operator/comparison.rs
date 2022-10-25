use guano_lexer::Token;

use crate::parser::ParseContext;

use super::ParseOperator;

#[derive(Debug, Clone)]
pub enum ComparisonOperator {
    GreaterThan,
    GreaterThanEquals,
    LessThan,
    LessThanEquals,
    Equals,
    NotEquals,
}

impl std::fmt::Display for ComparisonOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ComparisonOperator::GreaterThan => ">",
            ComparisonOperator::GreaterThanEquals => ">=",
            ComparisonOperator::LessThan => "<",
            ComparisonOperator::LessThanEquals => "<=",
            ComparisonOperator::Equals => "==",
            ComparisonOperator::NotEquals => "!=",
        })
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
                ComparisonOperator::NotEquals
            }
            _ => {
                context.stream.reset_peek();
                return None;
            }
        };

        Some(operator)
    }
}
