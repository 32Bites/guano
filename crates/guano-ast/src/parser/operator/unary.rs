use guano_lexer::Token;
use serde::{Deserialize, Serialize};

use crate::parser::{
    token_stream::{Spanned, ToSpanned},
    ParseContext,
};

use super::{Operator, ParseOperator};

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
    fn parse(context: &mut ParseContext) -> Option<Spanned<Self>> {
        let operator = match context.stream.peek_token::<1>()[0] {
            Some(Token::Exclamation) => UnaryOperator::Not,
            Some(Token::Minus) => UnaryOperator::Negate,
            _ => {
                context.stream.reset_peek();
                return None;
            }
        };
        let span = context.stream.read_span::<1>()[0].clone()?;

        Some(operator.to_spanned(span))
    }
}
