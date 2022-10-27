use guano_lexer::Token;
use serde::{Serialize, Deserialize};

use crate::parser::ParseContext;

use super::{ParseOperator, Operator};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FactorOperator {
    Multiply,
    Divide,
    Modulo,
}

impl std::fmt::Display for FactorOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.code())
    }
}

impl Operator for FactorOperator {
    type Str = &'static str;

    fn name(&self) -> Self::Str {
        match self {
            FactorOperator::Multiply => "multiplication",
            FactorOperator::Divide => "division",
            FactorOperator::Modulo => "modulo",
        }
    }

    fn code(&self) -> Self::Str {
        match self {
            FactorOperator::Multiply => "*",
            FactorOperator::Divide => "/",
            FactorOperator::Modulo => "%",
        }
    }
}

impl ParseOperator for FactorOperator {
    fn parse(context: &mut ParseContext) -> Option<Self> {
        let operator = match &context.stream.peek_token::<1>()[0] {
            Some(Token::Asterisk) => FactorOperator::Multiply,
            Some(Token::Slash) => FactorOperator::Divide,
            Some(Token::Percent) => FactorOperator::Modulo,
            _ => {
                context.stream.reset_peek();
                return None;
            }
        };
        context.stream.read::<1>();
        Some(operator)
    }
}
