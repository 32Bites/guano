use guano_lexer::Token;
use thiserror::Error;

use crate::parser::{
    error::{ParseError, ParseResult, ToParseError, ToParseResult},
    identifier::{Identifier, IdentifierError},
    typing::{Type, TypeError},
    Parse, ParseContext,
};

use super::{
    display::Display,
    literal::{Literal, LiteralError},
    operator::{
        BitwiseOperator, ComparisonOperator, EqualityOperator, FactorOperator,
        LogicalOperator, TermOperator, UnaryOperator,
    },
    simplify::Simplify,
};

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
        operator: UnaryOperator,
        right: Box<Expression>,
    },
    Factor {
        operator: FactorOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Term {
        operator: TermOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Comparison {
        operator: ComparisonOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Equality {
        operator: EqualityOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Bitwise {
        operator: BitwiseOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Logical {
        operator: LogicalOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
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
    fn parse(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError>
    {
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

    pub fn logical(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError>
    {
        let mut left = Expression::equality(context)?;

        loop {
            let operator = match context.stream.peek_token::<2>() {
                [Some(Token::Pipe), Some(Token::Pipe)] => LogicalOperator::Or,
                [Some(Token::Ampersand), Some(Token::Ampersand)] => LogicalOperator::And,
                _ => {
                    context.stream.reset_peek();
                    break;
                }
            };
            context.stream.read::<2>();

            let right = Box::new(Expression::equality(context)?);

            left = Expression::Logical {
                operator,
                left: Box::new(left),
                right,
            }
            .simplify_logical(context.simplified_expressions);
        }

        Ok(left)
    }

    pub fn equality(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError>
    {
        let mut left = Expression::comparison(context)?;

        loop {
            let operator = match context.stream.peek_token::<2>() {
                [Some(Token::Equals), Some(Token::Equals)] => EqualityOperator::Equals,
                [Some(Token::Exclamation), Some(Token::Equals)] => EqualityOperator::NotEquals,
                _ => {
                    context.stream.reset_peek();
                    break;
                }
            };

            context.stream.read::<2>();

            let right = Box::new(Expression::comparison(context)?);

            left = Expression::Equality {
                operator,
                left: Box::new(left),
                right,
            }
            .simplify_equality(context.simplified_expressions);
        }

        Ok(left)
    }

    pub fn comparison(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError>
    {
        let mut left = Expression::binary_bitwise(context)?;

        loop {
            let operator = match context.stream.peek_token::<2>() {
                [Some(Token::GreaterThan), n] if !matches!(n, Some(Token::GreaterThan)) => {
                    match n {
                        Some(Token::Equals) => {
                            context.stream.read::<2>();
                            ComparisonOperator::GreaterThanEquals
                        }
                        _ => {
                            context.stream.read::<1>();
                            ComparisonOperator::GreaterThan
                        }
                    }
                }
                [Some(Token::LessThan), n] if !matches!(n, Some(Token::LessThan)) => match n {
                    Some(Token::Equals) => {
                        context.stream.read::<2>();
                        ComparisonOperator::LessThanEquals
                    }
                    _ => {
                        context.stream.read::<1>();
                        ComparisonOperator::LessThan
                    }
                },
                _ => {
                    context.stream.reset_peek();
                    break;
                }
            };

            let right = Box::new(Expression::binary_bitwise(context)?);

            left = Expression::Comparison {
                operator,
                left: Box::new(left),
                right,
            }
            .simplify_comparison(context.simplified_expressions);
        }

        Ok(left)
    }

    pub fn binary_bitwise(
        context: &mut ParseContext,
    ) -> ParseResult<Expression, ExpressionError>
    {
        let mut left = Expression::bitshift(context)?;

        loop {
            let operator = match context.stream.peek_token::<2>() {
                [Some(Token::Pipe), o] if !matches!(o, Some(Token::Pipe)) => BitwiseOperator::Or,
                [Some(Token::Ampersand), o] if !matches!(o, Some(Token::Ampersand)) => {
                    BitwiseOperator::And
                }
                [Some(Token::Caret), _] => BitwiseOperator::Xor,
                _ => {
                    context.stream.reset_peek();
                    break;
                }
            };
            context.stream.read::<1>();

            let right = Box::new(Expression::bitshift(context)?);

            left = Expression::Bitwise {
                operator,
                left: Box::new(left),
                right,
            }
            .simplify_bitwise(context.simplified_expressions);
        }

        Ok(left)
    }

    pub fn bitshift(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError>
    {
        let mut left = Expression::term(context)?;

        loop {
            let operator = match context.stream.peek_token::<2>() {
                [Some(Token::LessThan), Some(Token::LessThan)] => BitwiseOperator::ShiftLeft,
                [Some(Token::GreaterThan), Some(Token::GreaterThan)] => BitwiseOperator::ShiftRight,
                _ => {
                    context.stream.reset_peek();
                    break;
                }
            };
            context.stream.read::<2>();

            let right = Box::new(Expression::term(context)?);

            left = Expression::Bitwise {
                operator,
                left: Box::new(left),
                right,
            }
            .simplify_bitwise(context.simplified_expressions);
        }

        Ok(left)
    }

    pub fn term(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError>
    {
        let mut left = Expression::factor(context)?;

        loop {
            let operator = match context.stream.peek_token::<1>()[0] {
                Some(Token::Plus) => TermOperator::Add,
                Some(Token::Minus) => TermOperator::Subtract,
                _ => {
                    context.stream.reset_peek();
                    break;
                }
            };
            context.stream.read::<1>();

            let right = Box::new(Expression::factor(context)?);

            left = Expression::Term {
                operator,
                left: Box::new(left),
                right,
            }
            .simplify_term(context.simplified_expressions);
        }

        Ok(left)
    }

    pub fn factor(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError>
    {
        let mut left = Expression::cast(context)?;

        loop {
            let operator = match &context.stream.peek_token::<1>()[0] {
                Some(Token::Asterisk) => FactorOperator::Multiply,
                Some(Token::Slash) => FactorOperator::Divide,
                _ => {
                    context.stream.reset_peek();
                    break;
                }
            };
            context.stream.read::<1>();

            let right = Box::new(Expression::cast(context)?);

            left = Expression::Factor {
                operator,
                left: Box::new(left),
                right,
            }
            .simplify_factor(context.simplified_expressions);
        }

        Ok(left)
    }

    pub fn cast(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError>
    {
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

    pub fn unary(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError>
    {
        let operator = match context.stream.peek_token::<1>()[0] {
            Some(Token::Exclamation) => UnaryOperator::LogicalNegate,
            Some(Token::Minus) => UnaryOperator::Negate,
            _ => {
                context.stream.reset_peek();
                return Expression::primary(context);
            }
        };
        context.stream.read::<1>();
        let right = Box::new(Expression::unary(context)?);

        Ok(Expression::Unary { operator, right }.simplify_unary(context.simplified_expressions))
    }

    pub fn primary(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError>
    {
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
    pub fn external(context: &mut ParseContext) -> ParseResult<Expression, IdentifierError>
    {
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
