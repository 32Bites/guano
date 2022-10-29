use std::str::FromStr;

use bigdecimal::{num_bigint::ParseBigIntError, BigDecimal, ParseBigDecimalError};

use guano_lexer::{escape_char::Token as EscapeToken, logos::Logos, Token};
use num::BigInt;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::parser::{
    error::{ParseError, ParseResult, ToParseError},
    token_stream::{Spanned, ToSpanned},
    Parse, ParseContext,
};

use super::{Expression, ExpressionKind};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Literal {
    String(String),
    Character(char),
    Integer(BigInt), // Signed or Unsigned
    FloatingPoint(BigDecimal),
    Boolean(bool),
    Nil,
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::String(s) => write!(f, "{s:?}"),
            Literal::Character(c) => write!(f, "{c:?}"),
            Literal::Integer(i) => write!(f, "{i}"),
            Literal::FloatingPoint(fp) => write!(f, "{fp}"),
            Literal::Boolean(b) => write!(f, "{b:?}"),
            Literal::Nil => f.write_str("nil"),
        }
    }
}

impl Parse<LiteralError, Spanned<Literal>> for Literal {
    fn parse(context: &mut ParseContext) -> ParseResult<Spanned<Literal>, LiteralError> {
        let literal = match &context.stream.read::<1>()[0] {
            Some((token, span)) => match token {
                Token::LitString(string) => {
                    let mut parsed_string = "".to_string();

                    for string_item in EscapeToken::lexer(&string) {
                        if let Some(character) = string_item.char() {
                            parsed_string.push(character)
                        } else {
                            return Err(LiteralError::InvalidString.to_parse_error(span.clone()));
                        }
                    }

                    Literal::String(parsed_string).to_spanned(span.clone())
                }

                Token::LitChar(character) => {
                    if character.len() == 0 {
                        return Err(
                            LiteralError::EmptyCharacterLiteral.to_parse_error(span.clone())
                        );
                    } else {
                        let mut escaper = EscapeToken::lexer(&character);

                        match (escaper.next(), escaper.next()) {
                            (None, None) | (Some(_), Some(_)) => {
                                return Err(
                                    LiteralError::InvalidCharacter.to_parse_error(span.clone())
                                );
                            }
                            (Some(token), None) => match token.char() {
                                Some(character) => {
                                    Literal::Character(character).to_spanned(span.clone())
                                }
                                None => {
                                    return Err(
                                        LiteralError::InvalidCharacter.to_parse_error(span.clone())
                                    );
                                }
                            },
                            (None, Some(_)) => unreachable!(),
                        }
                    }
                }

                Token::LitInteger(integer) => {
                    let decimal_string: String = integer.chars().filter(|c| *c != '_').collect();
                    let integer = match BigInt::from_str(&decimal_string) {
                        Ok(i) => i,
                        Err(e) => {
                            return Err(e.to_parse_error(span.clone()).convert());
                        }
                    };
                    Literal::Integer(integer).to_spanned(span.clone())
                }

                Token::LitBool(boolean) => Literal::Boolean(*boolean).to_spanned(span.clone()),

                Token::LitFloat(float) => {
                    let float_string: String = float.chars().filter(|c| *c != '_').collect();
                    let float = match BigDecimal::from_str(&float_string) {
                        Ok(f) => f,
                        Err(e) => {
                            return Err(e.to_parse_error(span.clone()).convert());
                        }
                    };
                    Literal::FloatingPoint(float).to_spanned(span.clone())
                }

                Token::LitNil => Literal::Nil.to_spanned(span.clone()),
                _ => {
                    return Err(ParseError::unexpected_token(span.clone()));
                }
            },
            None => return Err(ParseError::EndOfFile),
        };

        Ok(literal)
    }
}

impl Into<Expression> for Spanned<Literal> {
    fn into(self) -> Expression {
        Expression::new(ExpressionKind::Literal(self.value), self.span)
    }
}

#[derive(Debug, Error)]
pub enum LiteralError {
    #[error("invalid string literal")]
    InvalidString,
    #[error("empty character literal")]
    EmptyCharacterLiteral,
    #[error("invalid character literal")]
    InvalidCharacter,
    #[error("invalid integer literal")]
    InvalidInteger(#[from] ParseBigIntError),
    #[error("invalid floating point literal")]
    InvalidFloat(#[from] ParseBigDecimalError),
    #[error("not a literal")]
    InvalidLiteral,
}
