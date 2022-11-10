use std::fmt::Debug;

use owning_ref::RcRef;
use pest::{iterators::Pair};
use super::{parser::Rule, span::{Span, IntoSpan}};

#[derive(Debug, Clone)]
pub struct Operator<OpKind: Debug + Clone> {
    pub operator: OpKind,
    pub span: Span
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Negate,
    Not
}

impl Operator<UnaryOperator> {
    pub fn parse(pair: Pair<'_, Rule>, input: RcRef<str>) -> Self {
        let span = pair.as_span().into_span(input.clone());
        let operator = match pair.as_rule() {
            Rule::prefix => return Self::parse(pair.into_inner().next().unwrap(), input),
            Rule::negate => UnaryOperator::Negate,
            Rule::not => UnaryOperator::Not,
            _ => unreachable!()
        };

        Operator {
            span,
            operator
        }
    }
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Multiply,
    Divide,
    Modulo,
    Add,
    Subtract,
    
    BitshiftLeft,
    BitshiftRight,

    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,

    LogicalAnd,
    LogicalOr,

    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    Equal,
    NotEqual
}

impl Operator<BinaryOperator> {
    pub fn parse(pair: Pair<'_, Rule>, input: RcRef<str>) -> Self {
        let span = pair.as_span().into_span(input);
        let operator = match pair.as_rule() {
            Rule::mul => BinaryOperator::Multiply,
            Rule::div => BinaryOperator::Divide,
            Rule::modu => BinaryOperator::Modulo,
            Rule::add => BinaryOperator::Add,
            Rule::sub => BinaryOperator::Subtract,
            Rule::bitshift_left => BinaryOperator::BitshiftLeft,
            Rule::bitshift_right => BinaryOperator::BitshiftRight,
            Rule::bitwise_xor => BinaryOperator::BitwiseXor,
            Rule::bitwise_and => BinaryOperator::BitwiseAnd,
            Rule::bitwise_or => BinaryOperator::BitwiseOr,
            Rule::greater_than => BinaryOperator::GreaterThan,
            Rule::greater_than_eq => BinaryOperator::GreaterThanEqual,
            Rule::less_than => BinaryOperator::LessThan,
            Rule::less_than_eq => BinaryOperator::LessThanEqual,
            Rule::equals => BinaryOperator::Equal,
            Rule::not_equals => BinaryOperator::NotEqual,
            Rule::logical_and => BinaryOperator::LogicalAnd,
            Rule::logical_or => BinaryOperator::LogicalOr,
            _ => unreachable!()
        };

        Operator {
            span,
            operator
        }
    }
}

#[derive(Debug, Clone)]
pub enum AssignmentOperator {
    Assign,

    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,

    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,

    BitshiftLeft,
    BitshiftRight,

    LogicalAnd,
    LogicalOr
}

impl Operator<AssignmentOperator> {
    pub fn parse(pair: Pair<'_, Rule>, input: RcRef<str>) -> Self {
        let span = pair.as_span().into_span(input);
        let operator = match pair.as_str() {
            "=" => AssignmentOperator::Assign,
            "+=" => AssignmentOperator::Add,
            "-=" => AssignmentOperator::Subtract,
            "*=" => AssignmentOperator::Multiply,
            "/=" => AssignmentOperator::Divide,
            "%=" => AssignmentOperator::Modulo,
            "&=" => AssignmentOperator::BitwiseAnd,
            "|=" => AssignmentOperator::BitwiseOr,
            "^=" => AssignmentOperator::BitwiseXor,
            "<<=" => AssignmentOperator::BitshiftLeft,
            ">>=" => AssignmentOperator::BitshiftRight,
            "&&=" => AssignmentOperator::LogicalAnd,
            "||=" => AssignmentOperator::LogicalOr,
            _ => unreachable!()
        };

        Operator { operator, span }
    }
}