use guano_lexer::Token;
use thiserror::Error;

use crate::parser::{
    error::{ParseError, ParseResult, ToParseResult},
    expression::{Expression, ExpressionError},
    operator::{Assignment, ParseOperator},
    Parse, ParseContext,
};

use super::variable::{Variable, VariableError};

#[derive(Debug, Clone)]
pub enum Statement {
    Variable(Variable),
    Expression(Expression),
    Return(Option<Expression>),
    Assignment {
        left: Expression,
        operator: Assignment,
        right: Expression,
    },
    Empty,
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Variable(v) => v.fmt(f)?,
            Statement::Expression(e) => e.display().fmt(f)?,
            Statement::Return(r) => {
                f.write_str("return")?;
                if let Some(r) = r {
                    write!(f, " {}", r.display())?;
                }
            }
            Statement::Assignment {
                left,
                operator,
                right,
            } => write!(f, "{left} {operator} {right}")?,
            Statement::Empty => return Ok(()),
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

    // Ignore the random underscore, Rust's lexer does not seem to contextualize that `return`
    // in this circumstance refers not to the `return` keyword, but a function name... So the
    // underscore is just a hack to get the lexer to leave me alone ;-;
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
    ExpressionError(#[from] ExpressionError),
    #[error("expected semicolon")]
    MissingSemicolon,
}
