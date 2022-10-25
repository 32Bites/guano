use guano_lexer::Token;

use crate::parser::ParseContext;

use super::ParseOperator;

#[derive(Debug, Clone)]
pub enum FactorOperator {
    Multiply,
    Divide,
}

impl std::fmt::Display for FactorOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            FactorOperator::Multiply => "*",
            FactorOperator::Divide => "/",
        })
    }
}

impl ParseOperator for FactorOperator {
    fn parse(context: &mut ParseContext) -> Option<Self> {
        let operator = match &context.stream.peek_token::<1>()[0] {
            Some(Token::Asterisk) => FactorOperator::Multiply,
            Some(Token::Slash) => FactorOperator::Divide,
            _ => {
                context.stream.reset_peek();
                return None;
            }
        };
        context.stream.read::<1>();
        Some(operator)
    }
}
