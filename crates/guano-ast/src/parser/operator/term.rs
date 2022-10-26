use guano_lexer::Token;

use crate::parser::ParseContext;

use super::ParseOperator;

#[derive(Debug, Clone)]
pub enum TermOperator {
    Add,
    Subtract,
}

impl AsRef<str> for TermOperator {
    fn as_ref(&self) -> &str {
        match self {
            TermOperator::Add => "+",
            TermOperator::Subtract => "-",
        }
    }
}

impl std::fmt::Display for TermOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl ParseOperator for TermOperator {
    fn parse(context: &mut ParseContext) -> Option<Self> {
        let operator = match context.stream.peek_token::<1>()[0] {
            Some(Token::Plus) => TermOperator::Add,
            Some(Token::Minus) => TermOperator::Subtract,
            _ => {
                context.stream.reset_peek();
                return None;
            }
        };
        context.stream.read::<1>();

        Some(operator)
    }
}
