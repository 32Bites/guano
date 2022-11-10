pub mod conditional;

use owning_ref::RcRef;
use pest::{iterators::Pair};
use thiserror::Error;

use self::conditional::IfStatement;

use super::{
    super::parser::Rule,
    block::Block,
    declaration::variable::{VariableDeclaration, VariableError},
    expression::{Expression, ExpressionError},
    operator::{AssignmentOperator, Operator}, span::{SpanStr, Span, IntoSpan},
};

#[derive(Debug, Clone)]
pub enum StatementKind {
    Continue,
    Break,

    Return(Option<Expression>),
    Variable(VariableDeclaration),
    Expression(Expression),
    If(IfStatement),
    Loop(Block),
    Assignment {
        to: Expression,
        operator: Operator<AssignmentOperator>,
        with: Expression,
    },
    ForLoop {
        identifier: SpanStr,
        iterator: Expression,
        block: Block,
    },
    WhileLoop {
        condition: Expression,
        block: Block,
    },
}

#[derive(Debug, Clone)]
pub struct Statement {
    pub kind: StatementKind,
    pub span: Span,
}

impl Statement {
    pub fn parse(pair: Pair<'_, Rule>, input: RcRef<str>) -> Result<Self, StatementError> {
        let span = pair.as_span().into_span(input.clone());
        let kind = match pair.as_rule() {
            Rule::statement => return Self::parse(pair.into_inner().next().unwrap(), input),
            Rule::variable_declaration => StatementKind::Variable(VariableDeclaration::parse(pair, input)?),
            Rule::loop_control => {
                if pair.as_str() == "break" {
                    StatementKind::Break
                } else {
                    StatementKind::Continue
                }
            }
            Rule::return_statement => {
                let expression = match pair.into_inner().next() {
                    Some(inner) => Some(Expression::parse(inner.into_inner(), input)?),
                    _ => None,
                };

                StatementKind::Return(expression)
            }
            Rule::expression_statement => {
                let expression = Expression::parse(pair.into_inner().next().unwrap().into_inner(), input)?;
                StatementKind::Expression(expression)
            }
            Rule::assignment_statement => {
                let mut inner = pair.into_inner();

                let to = Expression::parse(inner.next().unwrap().into_inner(), input.clone())?;
                let operator = Operator::<AssignmentOperator>::parse(inner.next().unwrap(), input.clone());
                let with = Expression::parse(inner.next().unwrap().into_inner(), input)?;

                StatementKind::Assignment { to, operator, with }
            }
            Rule::infinite_loop => {
                let block = Block::parse(pair.into_inner().next().unwrap(), input)?;
                StatementKind::Loop(block)
            }
            Rule::for_loop => {
                let mut inner = pair.into_inner();
                let identifier = inner.next().unwrap().into_span_str(input.clone());
                let iterator = Expression::parse(inner.next().unwrap().into_inner(), input.clone())?;
                let block = Block::parse(inner.next().unwrap(), input)?;

                StatementKind::ForLoop {
                    identifier,
                    iterator,
                    block,
                }
            }
            Rule::while_loop => {
                let mut inner = pair.into_inner();
                let condition = Expression::parse(inner.next().unwrap().into_inner(), input.clone())?;
                let block = Block::parse(inner.next().unwrap(), input)?;

                StatementKind::WhileLoop { condition, block }
            }
            Rule::if_statement => StatementKind::If(IfStatement::parse(pair, input)?),
            _ => panic!("{:?} {:?}", pair.as_rule(), pair.as_str()),
        };

        Ok(Statement { kind, span })
    }
}

#[derive(Debug, Clone, Error)]
pub enum StatementError {
    #[error("{0}")]
    ExpressionError(#[from] ExpressionError),
    #[error("{0}")]
    VariableError(#[from] VariableError)
}
