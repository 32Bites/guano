use guano_lexer::Token;
use serde::{Deserialize, Serialize};

use crate::parser::{
    token_stream::{Spanned, ToSpanned},
    ParseContext,
};

use super::{Operator, ParseOperator};

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
    fn parse(context: &mut ParseContext) -> Option<Spanned<Self>> {
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
    fn parse(context: &mut ParseContext) -> Option<Spanned<BitwiseOperator>> {
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
        let span = context.stream.read_span::<1>()[0].clone()?;

        Some(operator.to_spanned(span))
    }
}

pub struct ShiftParser;

impl ParseOperator<BitwiseOperator> for ShiftParser {
    fn parse(context: &mut ParseContext) -> Option<Spanned<BitwiseOperator>> {
        let operator = match context.stream.peek_token::<2>() {
            [Some(Token::LessThan), Some(Token::LessThan)] => BitwiseOperator::ShiftLeft,
            [Some(Token::GreaterThan), Some(Token::GreaterThan)] => BitwiseOperator::ShiftRight,
            _ => {
                context.stream.reset_peek();
                return None;
            }
        };
        let span = context.stream.read_span_combined::<2>()?;

        Some(operator.to_spanned(span))
    }
}
