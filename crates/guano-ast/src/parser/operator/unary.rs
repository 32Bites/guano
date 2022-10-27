use guano_lexer::Token;
use serde::{Serialize, Deserialize};

use crate::parser::ParseContext;

use super::{ParseOperator, Operator};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnaryOperator {
    Negate,
    Not,
}

impl Operator for UnaryOperator {
    type Str = &'static str;
    
    fn name(&self) -> Self::Str {
        match self {
            UnaryOperator::Negate => "negate",
            UnaryOperator::Not => "not",
        }
    }

    fn code(&self) -> Self::Str {
        match self {
            UnaryOperator::Negate => "-",
            UnaryOperator::Not => "!",
        }
    }
}

impl std::fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.code())
    }
}

impl ParseOperator for UnaryOperator {
    fn parse(context: &mut ParseContext) -> Option<Self> {
        let operator = match context.stream.peek_token::<1>()[0] {
            Some(Token::Exclamation) => UnaryOperator::Not,
            Some(Token::Minus) => UnaryOperator::Negate,
            _ => {
                context.stream.reset_peek();
                return None;
            }
        };
        context.stream.read::<1>();

        Some(operator)
    }
}
