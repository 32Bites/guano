use crate::parser::{
    declaration::variable::VariableDeclaration,
    expression::Expression,
    operator::{AssignmentOperator, Operator},
    span::{Span, SpanStr},
    statement::{Statement as ParseStatement, StatementKind as ParseStatementKind},
};

use self::conditional::IfStatement;

use super::{block::Block, Desugar};

pub mod conditional;

#[derive(Debug, Clone)]
pub struct Statement {
    pub kind: StatementKind,
    pub span: Option<Span>,
}

impl Desugar for ParseStatement {
    type Unsweetened = Statement;

    fn desugar(self) -> Self::Unsweetened {
        Statement {
            span: Some(self.span),
            kind: self.kind.desugar(),
        }
    }
}

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
}

impl StatementKind {
    #[allow(unused_variables)]
    fn desugar_for(identifier: SpanStr, iterator: Expression, block: Block) -> Self {
        /*
        turn:

            for i in iter {
                ## Code here
            }

        into:
            let iterator = iter.into_iterator();
            loop {
                if iterator.has_next() {
                    let i = iterator.get_next();
                    ## Code here
                } else {
                    break;
                }
            }
        */
        todo!("For loops are not yet implemented, gotta figure out how iterators will work.")
    }

    /*
    turn:
        while condition {
            ## Code here
        }

    into:
        loop {
            if condition {
                ## Code here
            } else {
                break;
            }
        }
    */
    fn desugar_while(condition: Expression, block: Block) -> Self {
        let if_statement = Statement {
            kind: StatementKind::If(IfStatement {
                condition,
                block,
                else_block: [Statement {
                    kind: StatementKind::Break,
                    span: None,
                }]
                .into(),
                span: None,
            }),
            span: None,
        };
        StatementKind::Loop([if_statement].into())
    }
}

impl Desugar for ParseStatementKind {
    type Unsweetened = StatementKind;

    fn desugar(self) -> Self::Unsweetened {
        match self {
            ParseStatementKind::Continue => StatementKind::Continue,
            ParseStatementKind::Break => StatementKind::Break,
            ParseStatementKind::Return(r) => StatementKind::Return(r),
            ParseStatementKind::Variable(v) => StatementKind::Variable(v),
            ParseStatementKind::Expression(e) => StatementKind::Expression(e),
            ParseStatementKind::If(i) => StatementKind::If(i.desugar()),
            ParseStatementKind::Loop(l) => StatementKind::Loop(l.desugar()),
            ParseStatementKind::Assignment { to, operator, with } => {
                StatementKind::Assignment { to, operator, with }
            }
            ParseStatementKind::ForLoop {
                identifier,
                iterator,
                block,
            } => StatementKind::desugar_for(identifier, iterator, block.desugar()),
            ParseStatementKind::WhileLoop { condition, block } => {
                StatementKind::desugar_while(condition, block.desugar())
            }
        }
    }
}
