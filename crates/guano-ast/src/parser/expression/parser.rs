use guano_lexer::Token;
use thiserror::Error;

use crate::parser::{
    error::{ParseError, ParseResult, ToParseError, ToParseResult},
    identifier::{Identifier, IdentifierError},
    operator::{
        bitwise::{BitwiseParser, ShiftParser},
        Bitwise, Comparison, Factor, Logical, ParseOperator, Term, Unary,
    },
    typing::{Type, TypeError},
    Parse, ParseContext,
};

use super::{
    display::Display,
    literal::{Literal, LiteralError},
    simplify::Simplify,
};

#[derive(Debug, Clone)]
pub struct BinaryExpression<Operator: std::fmt::Debug + Clone> {
    pub left: Box<Expression>,
    pub operator: Operator,
    pub right: Box<Expression>,
}

impl<Operator: std::fmt::Debug + Clone> BinaryExpression<Operator> {
    pub fn new(operator: Operator, left: Expression, right: Expression) -> Self {
        Self {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Group(Box<Expression>),
    Literal(Literal),
    Variable(Identifier),
    Cast {
        left: Box<Expression>,
        cast_to: Type,
    },
    Unary {
        operator: Unary,
        right: Box<Expression>,
    },
    Factor(BinaryExpression<Factor>),
    Term(BinaryExpression<Term>),
    Comparison(BinaryExpression<Comparison>),
    Bitwise(BinaryExpression<Bitwise>),
    Logical(BinaryExpression<Logical>),
    /*     Format {
        format_string: String,
        arguments: Vec<Expression>,
    },
    FunctionCall {
        name: String,
        arguments: Vec<Expression>,
    },
    Access {
        owner: Box<Expression>,
        accessed_value: Box<Expression>, // TODO: Recursive Access, save for later.
    },
    Index {
        owner: Box<Expression>,
        index: Box<Expression>,
    }, */
}

impl Parse<ExpressionError> for Expression {
    fn parse(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
        Expression::logical(context)
    }
}

impl Expression {
    pub fn display(&self) -> Display<'_> {
        Display::new(self, false)
    }

    pub fn display_grouped(&self) -> Display<'_> {
        Display::new(self, true)
    }

    pub fn logical(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
        let mut left = Expression::comparison(context)?;

        while let Some(operator) = Logical::parse(context) {
            let right = Expression::comparison(context)?;

            left = Expression::Logical(BinaryExpression::new(operator, left, right))
                .simplify_logical(context.simplified_expressions);
        }

        Ok(left)
    }

    pub fn comparison(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
        let mut left = Expression::binary_bitwise(context)?;

        while let Some(operator) = Comparison::parse(context) {
            let right = Expression::binary_bitwise(context)?;

            left = Expression::Comparison(BinaryExpression::new(operator, left, right))
                .simplify_comparison(context.simplified_expressions);
        }

        Ok(left)
    }

    pub fn binary_bitwise(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
        let mut left = Expression::bitshift(context)?;

        while let Some(operator) = BitwiseParser::parse(context) {
            let right = Expression::bitshift(context)?;

            left = Expression::Bitwise(BinaryExpression::new(operator, left, right))
                .simplify_bitwise(context.simplified_expressions);
        }

        Ok(left)
    }

    pub fn bitshift(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
        let mut left = Expression::term(context)?;

        while let Some(operator) = ShiftParser::parse(context) {
            let right = Expression::term(context)?;

            left = Expression::Bitwise(BinaryExpression::new(operator, left, right))
                .simplify_bitwise(context.simplified_expressions);
        }

        Ok(left)
    }

    pub fn term(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
        let mut left = Expression::factor(context)?;

        while let Some(operator) = Term::parse(context) {
            let right = Expression::factor(context)?;

            left = Expression::Term(BinaryExpression::new(operator, left, right))
                .simplify_term(context.simplified_expressions);
        }

        Ok(left)
    }

    pub fn factor(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
        let mut left = Expression::cast(context)?;

        while let Some(operator) = Factor::parse(context) {
            let right = Expression::cast(context)?;

            left = Expression::Factor(BinaryExpression::new(operator, left, right))
                .simplify_factor(context.simplified_expressions);
        }

        Ok(left)
    }

    pub fn cast(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
        let mut left = Expression::unary(context)?;

        loop {
            match context.stream.peek_token::<1>()[0] {
                Some(Token::KeyAs) => {
                    context.stream.read::<1>();
                    let cast_type = Type::parse(context).to_parse_result()?;

                    left = Expression::Cast {
                        left: Box::new(left),
                        cast_to: cast_type,
                    }
                    .simplify_cast(context.simplified_expressions);
                }
                _ => {
                    context.stream.reset_peek();
                    break;
                }
            }
        }

        Ok(left)
    }

    pub fn unary(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
        if let Some(operator) = Unary::parse(context) {
            let right = Box::new(Expression::unary(context)?);

            Ok(
                Expression::Unary { operator, right }
                    .simplify_unary(context.simplified_expressions),
            )
        } else {
            Expression::primary(context)
        }
    }

    pub fn primary(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
        match &context.stream.peek::<1>()[0] {
            Some((token, span)) => match token {
                Token::OpenParen => {
                    context.stream.read::<1>();

                    let expression = Box::new(Expression::parse(context)?);

                    match &context.stream.read::<1>()[0] {
                        Some((Token::CloseParen, _)) => Ok(Expression::Group(expression)
                            .simplify_group(context.simplified_expressions)),
                        Some((_, span)) => {
                            Err(ExpressionError::MissingClosingParen.to_parse_error(span.clone()))
                        }
                        None => Err(ParseError::EndOfFile),
                    }
                }
                Token::LitNil
                | Token::LitBin(_)
                | Token::LitBool(_)
                | Token::LitChar(_)
                | Token::LitFloat(_)
                | Token::LitHex(_)
                | Token::LitInteger(_)
                | Token::LitString(_) => Ok(Expression::Literal(
                    Literal::parse(context).to_parse_result()?,
                )),

                Token::Identifier(_) => Ok(Expression::Variable(
                    Identifier::parse(context).to_parse_result()?,
                )),
                _ => {
                    context.stream.reset_peek();
                    return Err(ExpressionError::NotAnExpression.to_parse_error(span.clone()));
                }
            },
            None => Err(ParseError::EndOfFile),
        }
    }

    /// This will attempt to parse references to variables, function calls, etc, in the future.
    pub fn external(context: &mut ParseContext) -> ParseResult<Expression, IdentifierError> {
        // TODO: Currently only handles basic identifiers
        // I.E: no function calls, or paths (when they're a thing), just variable names.

        Ok(Expression::Variable(Identifier::parse(context)?))
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display().fmt(f)
    }
}

#[derive(Debug, Error)]
pub enum ExpressionError {
    #[error("{0}")]
    InvalidType(#[from] TypeError),
    #[error("{0}")]
    InvalidLiteral(#[from] LiteralError),
    #[error("{0}")]
    InvalidIdentifier(#[from] IdentifierError),
    #[error("expected expression")]
    ExpectedExpression,
    #[error("must close group with closing parenthesis")]
    MissingClosingParen,
    #[error("invalid expression")]
    NotAnExpression,
}
