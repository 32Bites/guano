use guano_lexer::{Span, Token};
use thiserror::Error;

use super::{Parse, Parser};

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    String,
    Character,
    Integer,
    UnsignedInteger,
    Boolean,
    FloatingPoint,
    Custom(String),
    Slice(Box<Type>),
}

impl<I: Iterator<Item = (Token, Span)>> Parse<I, TypeError> for Type {
    fn parse(parser: &mut Parser<I>) -> Result<Type, Option<TypeError>> {
        if let Some((token, _)) = parser.lexer.peek() {
            let output = match token {
                Token::PrimStr => Type::String,
                Token::PrimChar => Type::Character,
                Token::PrimUnsignedInteger => Type::UnsignedInteger,
                Token::PrimInteger => Type::Integer,
                Token::PrimFloat => Type::FloatingPoint,
                Token::PrimBool => Type::Boolean,
                // Token::Identifier(i) => Type::Custom(i.clone()),
                Token::Identifier(_) => {
                    parser.lexer.reset_peek();
                    return Err(Some(TypeError::CustomTypingNotAvailable));
                }
                Token::OpenBracket => {
                    parser.lexer.next();
                    if let Some((Token::CloseBracket, _)) = parser.lexer.next() {
                        let sub_type = Type::parse(parser)?;
                        return Ok(Type::Slice(Box::new(sub_type)));
                    } else {
                        return Err(None);
                    }
                }
                _ => {
                    parser.lexer.reset_peek();
                    return Err(None);
                }
            };
            parser.lexer.next();
            Ok(output)
        } else {
            parser.lexer.reset_peek();
            Err(None)
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
            Type::Slice(t) => return write!(f, "[]{t}"),
            Type::Custom(c) => return write!(f, "{c}"),
        })
    }
}

#[derive(Debug, Clone, Error)]
pub enum TypeError {
    #[error("custom types not implemented as of yet")]
    CustomTypingNotAvailable,
}
