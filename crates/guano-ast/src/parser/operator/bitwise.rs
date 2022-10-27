use guano_lexer::Token;
use serde::{Serialize, Deserialize};

use crate::parser::ParseContext;

use super::{ParseOperator, Operator};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BitwiseOperator {
    ShiftLeft,
    ShiftRight,
    Or,
    Xor,
    And,
}

impl std::fmt::Display for BitwiseOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.code())
    }
}

impl ParseOperator for BitwiseOperator {
    fn parse(context: &mut ParseContext) -> Option<Self> {
        BitwiseParser::parse(context).or_else(|| ShiftParser::parse(context))
    }
}

impl Operator for BitwiseOperator {
    type Str = &'static str;

    fn name(&self) -> Self::Str {
        match self {
            BitwiseOperator::ShiftLeft => "left bitshift",
            BitwiseOperator::ShiftRight => "right bitshift",
            BitwiseOperator::Or => "bitwise or",
            BitwiseOperator::Xor => "bitwise exclusive or",
            BitwiseOperator::And => "bitwise and",
        }
    }

    fn code(&self) -> Self::Str {
        match self {
            BitwiseOperator::ShiftLeft => "<<",
            BitwiseOperator::ShiftRight => ">>",
            BitwiseOperator::Or => "|",
            BitwiseOperator::Xor => "^",
            BitwiseOperator::And => "&",
        }
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
