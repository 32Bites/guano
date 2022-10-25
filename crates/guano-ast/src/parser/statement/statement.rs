use guano_lexer::Token;
use thiserror::Error;

use crate::parser::{
    error::{ParseError, ParseResult, ToParseError},
    expression::{Expression, ExpressionError},
    Parse, ParseContext,
};

use super::variable::{VariableDeclaration, VariableError};

#[derive(Debug, Clone)]
pub enum Statement {
    Variable(VariableDeclaration),
    Expression(Expression),
    Return(Option<Expression>),
    Empty,
}

impl Parse<StatementError> for Statement {
    fn parse(
        parser: &mut ParseContext,
    ) -> ParseResult<Statement, StatementError> {
        match &parser.stream.peek::<1>()[0] {
            Some((first_token, span)) => match first_token {
                Token::KeyVar | Token::KeyLet => todo!(),
                Token::KeyReturn => todo!(),
                Token::Semicolon => todo!(),
                _ => todo!(),
            },
            None => return Err(ParseError::EndOfFile),
        }
    }
}

impl Statement {
    fn variable(
        parser: &mut ParseContext,
    ) -> ParseResult<Statement, StatementError> {
        match VariableDeclaration::parse(parser) {
            Ok(statement) => Ok(Statement::Variable(statement)),
            Err(error) => match error {
                ParseError::Spanned(Some(VariableError::InvalidMutability), _)
                | ParseError::Unspanned(Some(VariableError::InvalidMutability))
                | ParseError::EndOfFile => Statement::expression(parser),
                _ => Err(error.convert()),
            },
        }
    }

    fn expression(
        parser: &mut ParseContext,
    ) -> ParseResult<Statement, StatementError> {
        match Expression::parse(parser) {
            Ok(expression) => Ok(Statement::Expression(expression)),
            Err(error) => match error {
                ParseError::Spanned(Some(ExpressionError::NotAnExpression), _)
                | ParseError::Unspanned(Some(ExpressionError::NotAnExpression))
                | ParseError::EndOfFile => Statement::return_(parser),
                _ => Err(error.convert()),
            },
        }
    }

    // Ignore the random underscore, Rust's lexer does not seem to contextualize that `return`
    // in this circumstance refers not to the `return` keyword, but a function name... So the
    // underscore is just a hack to get the lexer to leave me alone ;-;
    fn return_(
        parser: &mut ParseContext,
    ) -> ParseResult<Statement, StatementError> {
        match parser.stream.read_token::<1>()[0] {
            Some(Token::KeyReturn) => {
                let expression = Expression::parse(parser).ok();
                Ok(Statement::Return(expression))
            }
            Some(_) => Err(StatementError::InvalidReturnStart.to_parse_error(None)),
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
    #[error("invalid start to return statement")]
    InvalidReturnStart,
}
