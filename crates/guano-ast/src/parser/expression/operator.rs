use guano_lexer::{Span, Token};

use crate::parser::{ConvertResult, Parse, Parser};

use super::parser::ExpressionError;

#[derive(Debug, Clone)]
pub enum LogicalOperator {
    And,
    Or,
}

impl<I: Iterator<Item = (Token, Span)>> Parse<I, OperatorError> for LogicalOperator {
    fn parse(parser: &mut Parser<I>) -> Result<Self, Option<OperatorError>> {
        let operator = match parser.peek_token::<2>() {
            [Some(Token::Pipe), Some(Token::Pipe)] => LogicalOperator::Or,
            [Some(Token::Ampersand), Some(Token::Ampersand)] => LogicalOperator::And,
            _ => {
                parser.reset_peek();
                return Err(None);
            }
        };

        parser.read::<2>();

        Ok(operator)
    }
}

impl std::fmt::Display for LogicalOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            LogicalOperator::And => "&&",
            LogicalOperator::Or => "||",
        })
    }
}

#[derive(Debug)]
pub struct BitwiseOperationParser;

impl<I: Iterator<Item = (Token, Span)>> Parse<I, OperatorError, BitwiseOperator>
    for BitwiseOperationParser
{
    fn parse(parser: &mut Parser<I>) -> Result<BitwiseOperator, Option<OperatorError>> {
        let operator = match parser.peek_token::<2>() {
            [Some(Token::Pipe), o] if !matches!(o, Some(Token::Pipe)) => BitwiseOperator::Or,
            [Some(Token::Ampersand), o] if !matches!(o, Some(Token::Ampersand)) => {
                BitwiseOperator::And
            }
            [Some(Token::Caret), _] => BitwiseOperator::Xor,
            _ => {
                parser.reset_peek();
                return Err(None);
            }
        };

        parser.read::<1>();

        Ok(operator)
    }
}

#[derive(Debug)]
pub struct BitShiftParser;

impl<I: Iterator<Item = (Token, Span)>> Parse<I, OperatorError, BitwiseOperator>
    for BitShiftParser
{
    fn parse(parser: &mut Parser<I>) -> Result<BitwiseOperator, Option<OperatorError>> {
        let operator = match parser.peek_token::<2>() {
            [Some(Token::LessThan), Some(Token::LessThan)] => BitwiseOperator::ShiftLeft,
            [Some(Token::GreaterThan), Some(Token::GreaterThan)] => BitwiseOperator::ShiftRight,
            _ => {
                parser.reset_peek();
                return Err(None);
            }
        };

        parser.read::<2>();

        Ok(operator)
    }
}

#[derive(Debug, Clone)]
pub enum BitwiseOperator {
    ShiftLeft,
    ShiftRight,
    Or,
    Xor,
    And,
}

impl<I: Iterator<Item = (Token, Span)>> Parse<I, OperatorError> for BitwiseOperator {
    fn parse(parser: &mut Parser<I>) -> Result<Self, Option<OperatorError>> {
        BitShiftParser::parse(parser).or_else(|_| BitwiseOperationParser::parse(parser))
    }
}

impl std::fmt::Display for BitwiseOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            BitwiseOperator::ShiftLeft => "<<",
            BitwiseOperator::ShiftRight => ">>",
            BitwiseOperator::Or => "|",
            BitwiseOperator::Xor => "^",
            BitwiseOperator::And => "&",
        })
    }
}

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

impl<I: Iterator<Item = (Token, Span)>> Parse<I, OperatorError> for UnaryOperator {
    fn parse(parser: &mut Parser<I>) -> Result<Self, Option<OperatorError>> {
        let operator = match parser.peek_token::<1>() {
            [Some(Token::Minus)] => UnaryOperator::Negate,
            [Some(Token::Exclamation)] => UnaryOperator::LogicalNegate,
            _ => {
                parser.reset_peek();
                return Err(None);
            }
        };

        parser.read::<1>();

        Ok(operator)
    }
}

#[derive(Debug, Clone)]
pub enum EqualityOperator {
    Equals,
    NotEquals,
}

impl std::fmt::Display for EqualityOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            EqualityOperator::Equals => "==",
            EqualityOperator::NotEquals => "!=",
        })
    }
}

impl<I: Iterator<Item = (Token, Span)>> Parse<I, OperatorError> for EqualityOperator {
    fn parse(parser: &mut Parser<I>) -> Result<Self, Option<OperatorError>> {
        let operator = match parser.peek_token::<2>() {
            [Some(Token::Equals), Some(Token::Equals)] => EqualityOperator::Equals,
            [Some(Token::Exclamation), Some(Token::Equals)] => EqualityOperator::NotEquals,
            _ => {
                parser.reset_peek();
                return Err(None);
            }
        };

        parser.read_token::<2>();

        Ok(operator)
    }
}

#[derive(Debug, Clone)]
pub enum ComparisonOperator {
    GreaterThan,
    GreaterThanEquals,
    LessThan,
    LessThanEquals,
}

impl std::fmt::Display for ComparisonOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ComparisonOperator::GreaterThan => ">",
            ComparisonOperator::GreaterThanEquals => ">=",
            ComparisonOperator::LessThan => "<",
            ComparisonOperator::LessThanEquals => "<=",
        })
    }
}

impl<I: Iterator<Item = (Token, Span)>> Parse<I, OperatorError> for ComparisonOperator {
    fn parse(parser: &mut Parser<I>) -> Result<Self, Option<OperatorError>> {
        let operator = match parser.peek_token::<2>() {
            [Some(t @ (Token::GreaterThan | Token::LessThan)), Some(Token::Equals)] => {
                let operator = match t {
                    Token::GreaterThan => ComparisonOperator::GreaterThanEquals,
                    Token::LessThan => ComparisonOperator::LessThanEquals,
                    _ => unreachable!(),
                };
                parser.read_token::<2>();
                operator
            }
            [Some(Token::LessThan), o] if !matches!(o, Some(Token::LessThan)) => {
                parser.read_token::<1>();
                ComparisonOperator::LessThan
            }
            [Some(Token::GreaterThan), o] if !matches!(o, Some(Token::GreaterThan)) => {
                parser.read_token::<1>();
                ComparisonOperator::GreaterThan
            }
            _ => {
                parser.reset_peek();
                return Err(None);
            }
        };

        Ok(operator)
    }
}

#[derive(Debug, Clone)]
pub enum TermOperator {
    Add,
    Subtract,
}

impl std::fmt::Display for TermOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            TermOperator::Add => "+",
            TermOperator::Subtract => "-",
        })
    }
}

impl<I: Iterator<Item = (Token, Span)>> Parse<I, OperatorError> for TermOperator {
    fn parse(parser: &mut Parser<I>) -> Result<Self, Option<OperatorError>> {
        let operator = match parser.peek_token::<1>() {
            [Some(Token::Plus)] => TermOperator::Add,
            [Some(Token::Minus)] => TermOperator::Subtract,
            _ => {
                parser.reset_peek();
                return Err(None);
            }
        };

        parser.read::<1>();

        Ok(operator)
    }
}

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

impl<I: Iterator<Item = (Token, Span)>> Parse<I, OperatorError> for FactorOperator {
    fn parse(parser: &mut Parser<I>) -> Result<Self, Option<OperatorError>> {
        let operator = match &parser.peek_token::<1>() {
            [Some(Token::Asterisk)] => FactorOperator::Multiply,
            [Some(Token::Slash)] => FactorOperator::Divide,
            _ => {
                parser.reset_peek();
                return Err(None);
            }
        };

        parser.read::<1>();

        Ok(operator)
    }
}

#[derive(Debug)]
pub struct OperatorError;

impl std::fmt::Display for OperatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("operator error, this should be unreachable")
    }
}

impl std::error::Error for OperatorError {}

impl<T> ConvertResult<T, ExpressionError> for Result<T, Option<OperatorError>> {
    fn convert_result(self) -> Result<T, Option<ExpressionError>> {
        self.map_err(|_| None)
    }
}
