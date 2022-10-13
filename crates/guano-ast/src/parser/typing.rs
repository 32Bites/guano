use guano_lexer::{Span, Token};

use super::{Parse, Parser};

#[derive(Debug, Clone)]
pub enum Type {
    String,
    Character,
    Integer,
    UnsignedInteger,
    Boolean,
    FloatingPoint,
    Custom(String),
}

impl Parse for Type {
    fn parse(parser: &mut Parser<impl Iterator<Item = (Token, Span)>>) -> Option<Self> {
        if let Some((token, _)) = parser.lexer.peek() {
            let output = match token {
                Token::PrimStr => Type::String,
                Token::PrimChar => Type::Character,
                Token::PrimUnsignedInteger => Type::UnsignedInteger,
                Token::PrimInteger => Type::Integer,
                Token::PrimFloat => Type::FloatingPoint,
                Token::PrimBool => Type::Boolean,
                Token::Identifier(i) => Type::Custom(i.clone()),
                _ => {
                    parser.lexer.reset_peek();
                    return None;
                }
            };

            parser.lexer.next();

            Some(output)
        } else {
            None
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
            Type::Custom(c) => return write!(f, "{c}"),
        })
    }
}
