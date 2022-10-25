use guano_lexer::Token;

use crate::parser::ParseContext;

use super::ParseOperator;

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Negate,
    LogicalNegate,
}

impl std::fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            UnaryOperator::Negate => "-",
            UnaryOperator::LogicalNegate => "!",
        })
    }
}

impl ParseOperator for UnaryOperator {
    fn parse(context: &mut ParseContext) -> Option<Self> {
        let operator = match context.stream.peek_token::<1>()[0] {
            Some(Token::Exclamation) => UnaryOperator::LogicalNegate,
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
