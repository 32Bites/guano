use owning_ref::RcRef;
use pest::{iterators::Pair, pratt_parser::Op};

use crate::parser::{block::Block, expression::Expression, Rule, span::{Span, IntoSpan}};

use super::StatementError;

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub condition: Expression,
    pub block: Block,
    pub else_block: Option<ElseStatement>,
    pub span: Span,
}

impl IfStatement {
    pub fn parse(pair: Pair<'_, Rule>, input: RcRef<str>) -> Result<Self, StatementError> {
        let span = pair.as_span().into_span(input.clone());
        let mut inner = pair.into_inner();
        
        let condition = Expression::parse(inner.next().unwrap().into_inner(), input.clone())?;
        let block = Block::parse(inner.next().unwrap(), input.clone())?;

        let else_block = match inner.next() {
            Some(inner) => Some(ElseStatement::parse(inner, input)?),
            _ => None,
        };

        Ok(IfStatement {
            condition,
            block,
            else_block,
            span,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ElseStatement {
    pub else_type: ElseKind,
    pub span: Span,
}

impl ElseStatement {
    pub fn parse(pair: Pair<'_, Rule>, input: RcRef<str>) -> Result<Self, StatementError> {
        let span = pair.as_span().into_span(input.clone());

        let inner = pair.into_inner().next().unwrap();
        let else_type = match inner.as_rule() {
            Rule::block => ElseKind::Block(Block::parse(inner, input)?),
            Rule::if_statement => ElseKind::If(Box::new(IfStatement::parse(inner, input)?)),
            _ => unreachable!(),
        };

        Ok(
            ElseStatement { else_type, span }
        )
    }
}

#[derive(Debug, Clone)]
pub enum ElseKind {
    Block(Block),
    If(Box<IfStatement>),
}
