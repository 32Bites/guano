use guano_lexer::Token;

use crate::parser::ParseContext;

use super::ParseOperator;

#[derive(Debug, Clone)]
pub enum BitwiseOperator {
    ShiftLeft,
    ShiftRight,
    Or,
    Xor,
    And,
}

impl AsRef<str> for BitwiseOperator {
    fn as_ref(&self) -> &str {
        match self {
            BitwiseOperator::ShiftLeft => "<<",
            BitwiseOperator::ShiftRight => ">>",
            BitwiseOperator::Or => "|",
            BitwiseOperator::Xor => "^",
            BitwiseOperator::And => "&",
        }
    }
}

impl std::fmt::Display for BitwiseOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl ParseOperator for BitwiseOperator {
    fn parse(context: &mut ParseContext) -> Option<Self> {
        BitwiseParser::parse(context).or_else(|| ShiftParser::parse(context))
    }
}

pub struct BitwiseParser;

impl ParseOperator<BitwiseOperator> for BitwiseParser {
    fn parse(context: &mut ParseContext) -> Option<BitwiseOperator> {
        let operator = match context.stream.peek_token::<2>() {
            [Some(Token::Pipe), o] if !matches!(o, Some(Token::Pipe)) => BitwiseOperator::Or,
            [Some(Token::Ampersand), o] if !matches!(o, Some(Token::Ampersand)) => {
                BitwiseOperator::And
            }
            [Some(Token::Caret), _] => BitwiseOperator::Xor,
            _ => {
                context.stream.reset_peek();
                return None;
            }
        };
        context.stream.read::<1>();

        Some(operator)
    }
}

pub struct ShiftParser;

impl ParseOperator<BitwiseOperator> for ShiftParser {
    fn parse(context: &mut ParseContext) -> Option<BitwiseOperator> {
        let operator = match context.stream.peek_token::<2>() {
            [Some(Token::LessThan), Some(Token::LessThan)] => BitwiseOperator::ShiftLeft,
            [Some(Token::GreaterThan), Some(Token::GreaterThan)] => BitwiseOperator::ShiftRight,
            _ => {
                context.stream.reset_peek();
                return None;
            }
        };
        context.stream.read::<2>();

        Some(operator)
    }
}
