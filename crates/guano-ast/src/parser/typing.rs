use guano_lexer::Token;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{
    error::{ParseError, ParseResult, ToParseError},
    token_stream::{MergeSpan, Span, Spannable},
    Parse, ParseContext,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Type {
    pub kind: TypeKind,
    pub span: Span,
}

impl Type {
    pub fn new(kind: TypeKind, span: Span) -> Type {
        Type { kind, span }
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}

impl Spannable for Type {
    fn get_span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TypeKind {
    String,
    Character,
    Integer,
    UnsignedInteger,
    Boolean,
    FloatingPoint,
    Custom(String),
    List(Box<Type>),
    Tuple(Vec<Type>),
}

impl Parse<TypeError> for Type {
    fn parse(context: &mut ParseContext) -> ParseResult<Type, TypeError> {
        match &context.stream.peek::<1>()[0] {
            Some((token, span)) => match token {
                Token::PrimStr
                | Token::PrimCharacter
                | Token::PrimUnsignedInteger
                | Token::PrimInteger
                | Token::PrimFloat
                | Token::PrimBool => Type::parse_primitive(context),
                Token::OpenBracket => Type::parse_list(context),
                Token::OpenParen => Type::parse_tuple(context),
                Token::Identifier(_) => {
                    context.stream.reset_peek();
                    Err(TypeError::CustomTypingNotAvailable.to_parse_error(span.clone()))
                }
                _ => {
                    context.stream.reset_peek();
                    Err(ParseError::unexpected_token(span.clone()))
                }
            },
            None => Err(ParseError::EndOfFile),
        }
    }
}

impl Type {
    fn parse_primitive(context: &mut ParseContext) -> ParseResult<Type, TypeError> {
        match &context.stream.read::<1>()[0] {
            Some((token, span)) => {
                let span = span.clone();
                let kind = match token {
                    Token::PrimStr => TypeKind::String,
                    Token::PrimCharacter => TypeKind::Character,
                    Token::PrimUnsignedInteger => TypeKind::UnsignedInteger,
                    Token::PrimInteger => TypeKind::Integer,
                    Token::PrimFloat => TypeKind::FloatingPoint,
                    Token::PrimBool => TypeKind::Boolean,
                    _ => return Err(ParseError::unexpected_token(span)),
                };

                Ok(Type::new(kind, span))
            }
            None => Err(ParseError::EndOfFile),
        }
    }

    fn parse_list(context: &mut ParseContext) -> ParseResult<Type, TypeError> {
        match &context.stream.read::<2>() {
            [Some((token, span)), Some((second_token, second_span))] => match token {
                Token::OpenBracket => match second_token {
                    Token::CloseBracket => {
                        let sub_type = Type::parse(context)?;
                        let span = span.merge(second_span).merge(&sub_type.span);

                        let kind = TypeKind::List(Box::new(sub_type));

                        Ok(Type::new(kind, span))
                    }
                    _ => Err(ParseError::unexpected_token(second_span.clone())),
                },
                _ => Err(ParseError::unexpected_token(span.clone())),
            },

            _ => Err(ParseError::EndOfFile),
        }
    }

    fn parse_tuple(context: &mut ParseContext) -> ParseResult<Type, TypeError> {
        match &context.stream.read::<1>()[0] {
            Some((token, span)) => match token {
                Token::OpenParen => {
                    let mut types = vec![];
                    let mut final_span = span.clone();

                    if let Some((Token::CloseParen, span)) = &context.stream.peek::<1>()[0] {
                        context.stream.read::<1>();
                        final_span = final_span.merge(span);
                    } else {
                        loop {
                            context.stream.reset_peek();
                            let sub_type = Type::parse(context)?;

                            final_span = final_span.merge(&sub_type.span);
                            types.push(sub_type);

                            match &context.stream.read::<1>()[0] {
                                Some((token, span)) => match token {
                                    Token::Comma => {
                                        final_span = final_span.merge(&span);
                                    }
                                    Token::CloseParen => {
                                        final_span = final_span.merge(&span);
                                        break;
                                    }
                                    _ => return Err(ParseError::unexpected_token(span.clone())),
                                },
                                None => return Err(ParseError::EndOfFile),
                            }
                        }
                    }

                    let kind = TypeKind::Tuple(types);
                    Ok(Type::new(kind, final_span))
                }
                _ => Err(ParseError::unexpected_token(span.clone())),
            },
            None => Err(ParseError::EndOfFile),
        }
    }
}

impl std::fmt::Display for TypeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            TypeKind::String => "string",
            TypeKind::Character => "char",
            TypeKind::Integer => "int",
            TypeKind::UnsignedInteger => "uint",
            TypeKind::Boolean => "boolean",
            TypeKind::FloatingPoint => "float",
            TypeKind::List(t) => return write!(f, "[]{}", t.kind),
            TypeKind::Custom(c) => return write!(f, "{c}"),
            TypeKind::Tuple(v) => {
                return write!(f, "({})", v.iter().map(|t| t.kind.to_string()).join(", "))
            }
        })
    }
}

#[derive(Debug, Error)]
pub enum TypeError {
    #[error("custom types not implemented as of yet")]
    CustomTypingNotAvailable,
    #[error("a second closing bracket is expected for list types `[]`")]
    MissingClosingBracket,
    #[error("unexpected start to type")]
    UnexpectedStart,
}
