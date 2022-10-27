use guano_lexer::Token;
use serde::{Serialize, Deserialize};

use crate::parser::ParseContext;

use super::{ParseOperator, Operator};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LogicalOperator {
    And,
    Or,
}

impl std::fmt::Display for LogicalOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.code())
    }
}

impl Operator for LogicalOperator {
    type Str = &'static str;

    fn name(&self) -> Self::Str {
        match self {
            LogicalOperator::And => "logical and",
            LogicalOperator::Or => "logical or",
        }
    }

    fn code(&self) -> Self::Str {
        match self {
            LogicalOperator::And => "&&",
            LogicalOperator::Or => "||",
        }
    }
}

impl ParseOperator for LogicalOperator {
    fn parse(context: &mut ParseContext) -> Option<Self> {
        let operator = match context.stream.peek_token::<2>() {
            [Some(Token::Pipe), Some(Token::Pipe)] => LogicalOperator::Or,
            [Some(Token::Ampersand), Some(Token::Ampersand)] => LogicalOperator::And,
            _ => {
                context.stream.reset_peek();
                return None;
            }
        };
        context.stream.read::<2>();

        Some(operator)
    }
}
