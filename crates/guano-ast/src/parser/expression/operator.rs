use std::ops::Range;

use guano_lexer::{Span, Token};

use crate::parser::{
    error::{ParseError, ParseResult, ToParseError},
    Parse, ParseContext,
};

use super::parser::ExpressionError;

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

#[derive(Debug, Clone)]
pub enum BitwiseOperator {
    ShiftLeft,
    ShiftRight,
    Or,
    Xor,
    And,
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

#[derive(Debug)]
pub struct OperatorError;

impl std::fmt::Display for OperatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("operator error, this should be unreachable")
    }
}

impl std::error::Error for OperatorError {}
