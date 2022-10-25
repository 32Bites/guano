use guano_lexer::Token;
use thiserror::Error;

use super::{
    error::{ParseError, ParseResult, ToParseError},
    Parse, ParseContext,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    String,
    Character,
    Integer,
    UnsignedInteger,
    Boolean,
    FloatingPoint,
    Custom(String),
    List(Box<Type>),
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
            Some((token, span)) => match token {
                Token::PrimStr => Ok(Type::String),
                Token::PrimCharacter => Ok(Type::Character),
                Token::PrimUnsignedInteger => Ok(Type::UnsignedInteger),
                Token::PrimInteger => Ok(Type::Integer),
                Token::PrimFloat => Ok(Type::FloatingPoint),
                Token::PrimBool => Ok(Type::Boolean),
                _ => Err(ParseError::unexpected_token(span.clone())),
            },
            None => Err(ParseError::EndOfFile),
        }
    }

    fn parse_list(context: &mut ParseContext) -> ParseResult<Type, TypeError> {
        match &context.stream.read::<2>() {
            [Some((token, span)), Some((second_token, second_span))] => match token {
                Token::OpenBrace => match second_token {
                    Token::CloseBrace => {
                        let sub_type = Box::new(Type::parse(context)?);
                        Ok(Type::List(sub_type))
                    }
                    _ => Err(ParseError::unexpected_token(second_span.clone())),
                },
                _ => Err(ParseError::unexpected_token(span.clone())),
            },

            _ => Err(ParseError::EndOfFile),
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Type::String => "string",
            Type::Character => "char",
            Type::Integer => "int",
            Type::UnsignedInteger => "uint",
            Type::Boolean => "boolean",
            Type::FloatingPoint => "float",
            Type::List(t) => return write!(f, "[]{t}"),
            Type::Custom(c) => return write!(f, "{c}"),
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
