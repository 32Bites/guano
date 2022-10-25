use guano_lexer::Token;

use crate::parser::ParseContext;

use super::ParseOperator;

#[derive(Debug, Clone)]
pub enum LogicalOperator {
    And,
    Or,
}

impl std::fmt::Display for LogicalOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            LogicalOperator::And => "&&",
            LogicalOperator::Or => "||",
        })
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
