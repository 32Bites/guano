pub mod literal;

mod access;
mod bitwise;
mod cast;
mod comparison;
mod external;
mod factor;
mod list;
mod logical;
mod paren;
mod term;
mod unary;

use guano_lexer::Token;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::parser::{
    error::{ParseError, ParseResult, ToParseError, ToParseResult},
    identifier::{Identifier, IdentifierError},
    operator::{Bitwise, Comparison, Factor, Logical, Term, Unary},
    typing::{Type, TypeError},
    Parse, ParseContext,
};

use self::{
    access::parse_access,
    external::parse_external,
    list::parse_list,
    literal::{Literal, LiteralError},
    logical::parse_logical,
    paren::parse_paren,
};

use super::display::Display;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    pub identifier: Identifier,
    pub arguments: Vec<Expression>,
}

impl FunctionCall {
    pub fn children(&self) -> Vec<&Expression> {
        self.arguments.iter().collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

    pub fn children(&self) -> Vec<&Expression> {
        vec![&self.left, &self.right]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Expression {
    Group(Box<Expression>),
    Tuple(Vec<Expression>),
    List(Vec<Expression>),
    Literal(Literal),
    Variable(Identifier),
    FunctionCall(FunctionCall),
    Factor(BinaryExpression<Factor>),
    Term(BinaryExpression<Term>),
    Comparison(BinaryExpression<Comparison>),
    Bitwise(BinaryExpression<Bitwise>),
    Logical(BinaryExpression<Logical>),
    Cast {
        value: Box<Expression>,
        new_type: Type,
    },
    Unary {
        operator: Unary,
        right: Box<Expression>,
    },
    Index {
        value: Box<Expression>,
        index: Box<Expression>,
    },
    Property {
        value: Box<Expression>,
        property: Identifier,
    },
    MethodCall {
        value: Box<Expression>,
        method: FunctionCall,
    },
    Format {
        format: String,
        with: Box<Expression>,
    },
}

impl Parse<ExpressionError> for Expression {
    fn parse(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
        parse_logical(context)
    }
}

impl Expression {
    pub fn display(&self) -> Display<'_> {
        Display::new(self, false)
    }

    pub fn display_grouped(&self) -> Display<'_> {
        Display::new(self, true)
    }

    pub fn children(&self) -> Vec<&Expression> {
        match self {
            Expression::Group(g) => vec![&g],
            Expression::Tuple(t) => t.iter().collect(),
            Expression::List(l) => l.iter().collect(),
            Expression::Literal(_) | Expression::Variable(_) => vec![],
            Expression::FunctionCall(f) => f.children(),
            Expression::Factor(f) => f.children(),
            Expression::Term(t) => t.children(),
            Expression::Comparison(c) => c.children(),
            Expression::Bitwise(b) => b.children(),
            Expression::Logical(l) => l.children(),
            Expression::Cast { value, new_type: _ } => vec![&value],
            Expression::Unary { operator: _, right } => vec![&right],
            Expression::Index { value, index } => vec![&value, &index],
            Expression::Property { value, property: _ } => vec![&value],
            Expression::MethodCall { value, method } => {
                [&**value].into_iter().chain(method.children()).collect()
            }
            Expression::Format { format: _, with } => vec![&with],
        }
    }

    pub fn primary(context: &mut ParseContext) -> ParseResult<Expression, ExpressionError> {
        match &context.stream.peek::<1>()[0] {
            Some((token, span)) => match token {
                Token::OpenParen => parse_paren(context),
                Token::LitNil
                | Token::LitBin(_)
                | Token::LitBool(_)
                | Token::LitChar(_)
                | Token::LitFloat(_)
                | Token::LitHex(_)
                | Token::LitInteger(_)
                | Token::LitString(_) => match Literal::parse(context).to_parse_result()? {
                    Literal::String(string) => {
                        if let Some(Token::Colon) = context.stream.peek_token::<1>()[0] {
                            context.stream.read::<1>();
                            // let with = Expression::parse(context)?; // parse entire expression.
                            let with = parse_access(context)?; // Parse either a primary or an accessor of a primary.

                            Ok(Expression::Format {
                                format: string,
                                with: Box::new(with),
                            })
                        } else {
                            context.stream.reset_peek();
                            Ok(Expression::Literal(Literal::String(string)))
                        }
                    }
                    l => Ok(Expression::Literal(l)),
                },

                Token::Identifier(_) => parse_external(context),
                Token::OpenBracket => parse_list(context),
                _ => {
                    context.stream.reset_peek();
                    return Err(ExpressionError::NotAnExpression.to_parse_error(span.clone()));
                }
            },
            None => Err(ParseError::EndOfFile),
        }
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
