use crate::{
    desugar::{block::Block, Desugar},
    parser::{
        expression::Expression,
        span::Span,
        statement::conditional::{ElseKind, ElseStatement, IfStatement as ParseIfStatement},
    },
};

use super::{Statement, StatementKind};

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub condition: Expression,
    pub block: Block,
    pub else_block: Block,
    pub span: Option<Span>,
}

impl Desugar for ParseIfStatement {
    type Unsweetened = IfStatement;

    fn desugar(self) -> Self::Unsweetened {
        IfStatement {
            condition: self.condition,
            block: self.block.desugar(),
            else_block: self
                .else_block
                .map_or_else(|| Block::default(), |e| e.desugar()),
            span: Some(self.span),
        }
    }
}

impl Desugar for ElseStatement {
    type Unsweetened = Block;

    fn desugar(self) -> Self::Unsweetened {
        match self.else_type {
            ElseKind::Block(b) => b.desugar(),
            ElseKind::If(s) => Block {
                items: vec![Statement {
                    span: Some(s.span.clone()),
                    kind: StatementKind::If(s.desugar()),
                }
                .into()],
                span: Some(self.span),
            },
        }
    }
}

impl Desugar for ElseKind {
    type Unsweetened = Block;

    fn desugar(self) -> Self::Unsweetened {
        match self {
            ElseKind::Block(b) => b.desugar(),
            ElseKind::If(s) => [Statement {
                kind: StatementKind::If(s.desugar()),
                span: None,
            }]
            .into(),
        }
    }
}
