use guano_lexer::{Span, Token};
use thiserror::Error;

use crate::{
    convert_result_impl,
    parser::{
        typing::{Type, TypeError},
        ConvertResult, Parse, Parser,
    },
};

use super::{
    display::Display,
    literal::{Literal, LiteralError},
    operator::{ComparisonOperator, EqualityOperator, FactorOperator, TermOperator, UnaryOperator},
    simplify::Simplify,
};

#[derive(Debug, Clone)]
pub enum Expression {
    Group(Box<Expression>),
    Literal(Literal),
    Variable(String),
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

impl<I: Iterator<Item = (Token, Span)>> Parse<I, ExpressionError> for Expression {
    fn parse(parser: &mut Parser<I>) -> Result<Expression, Option<ExpressionError>> {
        Expression::equality(parser)
    }
}

impl Expression {
    pub fn display(&self) -> Display<'_> {
        Display::new(self, false)
    }

    pub fn display_grouped(&self) -> Display<'_> {
        Display::new(self, true)
    }

    pub fn get_type(&self) -> Option<Type> {
        match self {
            Expression::Group(g) => g.get_type(),
            Expression::Literal(l) => l.get_type(),
            _ => None,
        }
    }

    pub fn equality(
        parser: &mut Parser<impl Iterator<Item = (Token, Span)>>,
    ) -> Result<Expression, Option<ExpressionError>> {
        let mut left = Expression::comparison(parser)?;
        loop {
            if let Ok(operator) = EqualityOperator::parse(parser) {
                let right = Box::new(Expression::comparison(parser)?);

                left = Expression::Equality {
                    operator,
                    left: Box::new(left),
                    right,
                }
                .simplify_equality();
            } else {
                parser.reset_peek();
                break;
            }
        }

        Ok(left)
    }

    pub fn comparison(
        parser: &mut Parser<impl Iterator<Item = (Token, Span)>>,
    ) -> Result<Expression, Option<ExpressionError>> {
        let mut left = Expression::cast(parser)?;

        loop {
            if let Ok(operator) = ComparisonOperator::parse(parser) {
                let right = Box::new(Expression::cast(parser)?);

                left = Expression::Comparison {
                    operator,
                    left: Box::new(left),
                    right,
                }
                .simplify_comparison();
            } else {
                parser.reset_peek();
                break;
            }
        }

        Ok(left)
    }

    pub fn cast(
        parser: &mut Parser<impl Iterator<Item = (Token, Span)>>,
    ) -> Result<Expression, Option<ExpressionError>> {
        let mut left = Expression::term(parser)?;
        loop {
            if let Some((Token::KeyAs, _)) = parser.lexer.peek() {
                parser.lexer.next();

                let cast_type = Type::parse(parser).convert_result()?;

                left = Expression::Cast {
                    left: Box::new(left),
                    cast_to: cast_type,
                }
                .simplify_cast();
            } else {
                parser.lexer.reset_peek();
                break;
            }
        }

        Ok(left)
    }

    pub fn term(
        parser: &mut Parser<impl Iterator<Item = (Token, Span)>>,
    ) -> Result<Expression, Option<ExpressionError>> {
        let mut left = Expression::factor(parser)?;
        loop {
            if let Ok(operator) = TermOperator::parse(parser) {
                let right = Box::new(Expression::factor(parser)?);

                left = Expression::Term {
                    operator,
                    left: Box::new(left),
                    right,
                }
                .simplify_term()
            } else {
                parser.reset_peek();
                break;
            }
        }

        Ok(left)
    }

    pub fn factor(
        parser: &mut Parser<impl Iterator<Item = (Token, Span)>>,
    ) -> Result<Expression, Option<ExpressionError>> {
        let mut left = Expression::unary(parser)?;

        loop {
            if let Ok(operator) = FactorOperator::parse(parser) {
                let right = Box::new(Expression::unary(parser)?);

                left = Expression::Factor {
                    operator,
                    left: Box::new(left),
                    right,
                }
                .simplify_factor();
            } else {
                parser.reset_peek();
                break;
            }
        }

        Ok(left)
    }

    pub fn unary(
        parser: &mut Parser<impl Iterator<Item = (Token, Span)>>,
    ) -> Result<Expression, Option<ExpressionError>> {
        let operator = match UnaryOperator::parse(parser) {
            Ok(operator) => operator,
            Err(_) => return Expression::primary(parser),
        };
        let right = Box::new(Expression::unary(parser)?);

        Ok(Expression::Unary { operator, right })
    }

    pub fn primary(
        parser: &mut Parser<impl Iterator<Item = (Token, Span)>>,
    ) -> Result<Expression, Option<ExpressionError>> {
        match Literal::parse(parser) {
            Ok(literal) => Ok(Expression::Literal(literal)),
            Err(None) => match Expression::external(parser) {
                Ok(_external) => todo!(),
                Err(None) => match Expression::group(parser) {
                    g @ Ok(_) => g,
                    Err(None) => Err(None),
                    Err(Some(error)) => error.convert_result(),
                },
                Err(Some(error)) => error.convert_result(),
            },
            Err(Some(error)) => error.convert_result(),
        }
    }

    pub fn group(
        parser: &mut Parser<impl Iterator<Item = (Token, Span)>>,
    ) -> Result<Expression, Option<ExpressionError>> {
        if let Some(Token::OpenParen) = parser.peek_token::<1>()[0] {
            parser.read::<1>();

            let expression = Box::new(Expression::parse(parser)?);

            if let Some(Token::CloseParen) = parser.read_token::<1>()[0] {
                return Ok(Expression::Group(expression).simplify_group());
            }
        } else {
            parser.reset_peek();
        }

        Err(None)
    }

    /// This will attempt to parse references to variables, function calls, etc, in the future.
    pub fn external(
        _parser: &mut Parser<impl Iterator<Item = (Token, Span)>>,
    ) -> Result<Expression, Option<ExpressionError>> {
        // TODO: Write functionality
        Err(None)
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
}

convert_result_impl!(ExpressionError);

#[cfg(test)]
mod tests {
    use crate::parser::{Parse, Parser};

    use super::Expression;

    #[test]
    fn test() {
        let test = "(((5 <= 6)))";

        let mut parser = <Parser>::from_source(test); // dafaq

        let expression = Expression::parse(&mut parser).unwrap();

        println!("Ungrouped: {}", expression.display());
        println!("Grouped: {}", expression.display_grouped());
        println!("Debug: {:?}", expression);
        println!("{:?}", parser.lexer.next());
    }
}
