use guano_lexer::Token;
use serde::{Deserialize, Serialize};

use crate::parser::{
    token_stream::{Spanned, ToSpanned},
    ParseContext,
};

use super::{Operator, ParseOperator};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TermOperator {
    Add,
    Subtract,
}

impl std::fmt::Display for TermOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.code())
    }
}

impl Operator for TermOperator {
    type Str = &'static str;

    fn name(&self) -> Self::Str {
        match self {
            TermOperator::Add => "addition",
            TermOperator::Subtract => "subtraction",
        }
    }

    fn code(&self) -> Self::Str {
        match self {
            TermOperator::Add => "+",
            TermOperator::Subtract => "-",
        }
    }
}

impl ParseOperator for TermOperator {
    fn parse(context: &mut ParseContext) -> Option<Spanned<Self>> {
        let operator = match context.stream.peek_token::<1>()[0] {
            Some(Token::Plus) => TermOperator::Add,
            Some(Token::Minus) => TermOperator::Subtract,
            _ => {
                context.stream.reset_peek();
                return None;
            }
        };
        let span = context.stream.read_span::<1>()[0].clone()?;

        Some(operator.to_spanned(span))
    }
}
