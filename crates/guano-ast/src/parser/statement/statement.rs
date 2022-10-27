use guano_lexer::Token;
use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::parser::{
    block::BlockError,
    error::{ParseError, ParseResult, ToParseResult},
    expression::{Expression, ExpressionError},
    identifier::IdentifierError,
    operator::{Assignment, ParseOperator},
    Parse, ParseContext,
};

use super::{
    conditional::Conditional,
    for_loop::ForLoop,
    variable::{Variable, VariableError},
    while_loop::WhileLoop,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Statement {
    Variable(Variable),
    Expression(Expression),
    Return(Option<Expression>),
    Assignment {
        left: Expression,
        operator: Assignment,
        right: Expression,
    },
    ForLoop(ForLoop),
    WhileLoop(WhileLoop),
    Conditional(Conditional),
    Empty,
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Variable(v) => v.fmt(f)?,
            Statement::Expression(e) => e.fmt(f)?,
            Statement::Return(r) => {
                f.write_str("return")?;
                if let Some(r) = r {
                    write!(f, " {r}")?;
                }
            }
            Statement::Assignment {
                left,
                operator,
                right,
            } => write!(f, "{left} {operator} {right}")?,
            Statement::Empty => return Ok(()),
            Statement::ForLoop(fl) => return fl.fmt(f),
            Statement::Conditional(c) => return c.fmt(f),
            Statement::WhileLoop(w) => return w.fmt(f),
        }

        f.write_str(";")
    }
}

impl Parse<StatementError> for Statement {
    fn parse(context: &mut ParseContext) -> ParseResult<Statement, StatementError> {
        let statement = match &context.stream.peek_token::<1>()[0] {
            Some(token) => match token {
                Token::KeyVar | Token::KeyLet => Statement::variable(context),
                Token::KeyReturn => Statement::return_(context),
                Token::Semicolon => Statement::empty(context),
                Token::KeyIf => return Statement::if_(context),
                Token::KeyFor => return Statement::for_(context),
                Token::KeyWhile => return Statement::while_(context),
                _ => Statement::expression_or_assignment(context),
            },
            None => Err(ParseError::EndOfFile),
        }?;

        match &context.stream.read::<1>()[0] {
            Some((token, span)) => match token {
                Token::Semicolon => Ok(statement),
                _ => Err(ParseError::unexpected_token(span.clone())),
            },
            None => Err(ParseError::EndOfFile),
        }
    }
}

impl Statement {
    fn variable(context: &mut ParseContext) -> ParseResult<Statement, StatementError> {
        Ok(Statement::Variable(
            Variable::parse(context).to_parse_result()?,
        ))
    }

    fn empty(context: &mut ParseContext) -> ParseResult<Statement, StatementError> {
        context.stream.reset_peek();

        while let [Some(Token::Semicolon), Some(Token::Semicolon)] =
            context.stream.peek_token::<2>()
        {
            context.stream.read::<1>();
        }
        context.stream.reset_peek();
        Ok(Statement::Empty)
    }

    fn expression_or_assignment(
        context: &mut ParseContext,
    ) -> ParseResult<Statement, StatementError> {
        context.stream.reset_peek();
        let left = Expression::parse(context).to_parse_result()?;
        if let Some(operator) = Assignment::parse(context) {
            let right = Expression::parse(context).to_parse_result()?;
            Ok(Statement::Assignment {
                left,
                operator,
                right,
            })
        } else {
            Ok(Statement::Expression(left))
        }
    }

    fn if_(context: &mut ParseContext) -> ParseResult<Statement, StatementError> {
        Ok(Statement::Conditional(Conditional::parse(context)?))
    }

    fn for_(context: &mut ParseContext) -> ParseResult<Statement, StatementError> {
        Ok(Statement::ForLoop(ForLoop::parse(context)?))
    }

    fn while_(context: &mut ParseContext) -> ParseResult<Statement, StatementError> {
        Ok(Statement::WhileLoop(WhileLoop::parse(context)?))
    }

    fn return_(context: &mut ParseContext) -> ParseResult<Statement, StatementError> {
        match &context.stream.read::<1>()[0] {
            Some((Token::KeyReturn, _)) => {
                let expression = match context.stream.peek_token::<1>()[0] {
                    Some(Token::Semicolon) => {
                        context.stream.reset_peek();
                        None
                    }
                    _ => Some(Expression::parse(context).to_parse_result()?),
                };

                Ok(Statement::Return(expression))
            }
            Some((_, span)) => Err(ParseError::unexpected_token(span.clone())),
            None => Err(ParseError::EndOfFile),
        }
    }
}

#[derive(Debug, Error)]
pub enum StatementError {
    #[error("{0}")]
    VariableError(#[from] VariableError),
    #[error("{0}")]
    IdentifierError(#[from] IdentifierError),
    #[error("{0}")]
    ExpressionError(#[from] ExpressionError),
    #[error("{0}")]
    BlockError(#[from] BlockError),
    #[error("expected semicolon")]
    MissingSemicolon,
}
