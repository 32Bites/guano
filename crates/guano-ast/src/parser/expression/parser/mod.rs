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
    token_stream::{MergeSpan, Span, Spannable, Spanned, ToSpanned},
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
pub struct Expression {
    pub span: Span,
    pub kind: ExpressionKind,
}

impl Spannable for Expression {
    fn get_span(&self) -> Span {
        self.span.clone()
    }
}

impl Expression {
    pub fn traverse<F: FnMut(&Expression)>(&self, mut func: F) {
        func(self);
        self.children().iter().for_each(|&e| {
            e.traverse(Box::new(&mut func) as Box<dyn FnMut(&Expression)>);
        })
    }

    pub fn new(kind: ExpressionKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn display(&self) -> Display<'_> {
        Display::new(self, false)
    }

    pub fn display_grouped(&self) -> Display<'_> {
        Display::new(self, true)
    }

    pub fn children(&self) -> Vec<&Expression> {
        match &self.kind {
            ExpressionKind::Group(g) => vec![&g],
            ExpressionKind::Tuple(t) => t.iter().collect(),
            ExpressionKind::List(l) => l.iter().collect(),
            ExpressionKind::Literal(_) | ExpressionKind::Variable(_) => vec![],
            ExpressionKind::FunctionCall(f) => f.children(),
            ExpressionKind::Factor(f) => f.children(),
            ExpressionKind::Term(t) => t.children(),
            ExpressionKind::Comparison(c) => c.children(),
            ExpressionKind::Bitwise(b) => b.children(),
            ExpressionKind::Logical(l) => l.children(),
            ExpressionKind::Cast { value, new_type: _ } => vec![&value],
            ExpressionKind::Unary { operator: _, right } => vec![&right],
            ExpressionKind::Index { value, index } => vec![&value, &index],
            ExpressionKind::Property { value, property: _ } => vec![&value],
            ExpressionKind::MethodCall { value, method } => {
                [&**value].into_iter().chain(method.children()).collect()
            }
            ExpressionKind::Format { format: _, with } => vec![&with],
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
                    Spanned {
                        value: Literal::String(string),
                        span,
                    } => {
                        if let Some((Token::Colon, colon_span)) = &context.stream.peek::<1>()[0] {
                            context.stream.read::<1>();
                            // let with = Expression::parse(context)?; // parse entire expression.
                            let with = parse_access(context)?; // Parse either a primary or an accessor of a primary.
                            let span = span.merge(colon_span).merge(&with.span);

                            let format = ExpressionKind::Format {
                                format: string,
                                with: Box::new(with),
                            };

                            Ok(Expression::new(format, span))
                        } else {
                            context.stream.reset_peek();
                            Ok(Literal::String(string).to_spanned(span).into())
                        }
                    }
                    l => Ok(l.into()),
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    pub identifier: Identifier,
    pub arguments: Vec<Expression>,
}

impl FunctionCall {
    pub fn children(&self) -> Vec<&Expression> {
        self.arguments.iter().collect()
    }

    pub fn children_mut(&mut self) -> Vec<&mut Expression> {
        self.arguments.iter_mut().collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryExpression<Operator: std::fmt::Debug + Clone> {
    pub left: Box<Expression>,
    pub operator: Spanned<Operator>,
    pub right: Box<Expression>,
}

impl<Operator: std::fmt::Debug + Clone> BinaryExpression<Operator> {
    pub fn new(operator: Spanned<Operator>, left: Expression, right: Expression) -> Self {
        Self {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn children(&self) -> Vec<&Expression> {
        vec![&self.left, &self.right]
    }

    pub fn children_mut(&mut self) -> Vec<&mut Expression> {
        vec![&mut self.left, &mut self.right]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpressionKind {
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
        operator: Spanned<Unary>,
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
