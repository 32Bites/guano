use std::{cmp::Ordering, ops::Range, str::FromStr};

use bigdecimal::{
    num_bigint::{ParseBigIntError, Sign, ToBigInt},
    BigDecimal, ParseBigDecimalError, ToPrimitive, Zero,
};
use guano_lexer::{escape_char::Token as EscapeToken, logos::Logos, Span, Token};
use num::BigInt;
use thiserror::Error;

use crate::parser::{
    error::{ParseError, ParseResult, ToParseError},
    typing::Type,
    Parse, ParseContext,
};

use super::parser::Expression;

#[derive(Debug, Clone)]
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
            Literal::FloatingPoint(fp) => write!(f, "{fp:?}"),
            Literal::Boolean(b) => write!(f, "{b:?}"),
            Literal::Nil => f.write_str("nil"),
        }
    }
}

impl Literal {
    pub fn to_expression(self) -> Expression {
        Expression::Literal(self)
    }

    pub fn bs_left(&self, rhs: &Self) -> Option<Self> {
        Some(match (self, rhs) {
            (Literal::Integer(lhs), Literal::Integer(rhs)) => {
                match (lhs.to_i128(), rhs.to_i128()) {
                    (Some(lhs), Some(rhs)) => Literal::Integer((lhs << rhs).to_bigint()?),
                    _ => match (lhs.to_u128(), rhs.to_u128()) {
                        (Some(lhs), Some(rhs)) => Literal::Integer((lhs << rhs).to_bigint()?),
                        _ => return None,
                    },
                }
            }
            _ => return None,
        })
    }

    pub fn bs_right(&self, rhs: &Self) -> Option<Self> {
        Some(match (self, rhs) {
            (Literal::Integer(lhs), Literal::Integer(rhs)) => {
                match (lhs.to_i128(), rhs.to_i128()) {
                    (Some(lhs), Some(rhs)) => Literal::Integer((lhs >> rhs).to_bigint()?),
                    _ => match (lhs.to_u128(), rhs.to_u128()) {
                        (Some(lhs), Some(rhs)) => Literal::Integer((lhs >> rhs).to_bigint()?),
                        _ => return None,
                    },
                }
            }
            _ => return None,
        })
    }

    pub fn b_and(&self, rhs: &Self) -> Option<Self> {
        Some(match (self, rhs) {
            (Literal::Integer(lhs), Literal::Integer(rhs)) => Literal::Integer(lhs & rhs),
            (Literal::Boolean(lhs), Literal::Boolean(rhs)) => Literal::Boolean(lhs & rhs),
            _ => return None,
        })
    }

    pub fn b_or(&self, rhs: &Self) -> Option<Self> {
        Some(match (self, rhs) {
            (Literal::Integer(lhs), Literal::Integer(rhs)) => Literal::Integer(lhs | rhs),
            (Literal::Boolean(lhs), Literal::Boolean(rhs)) => Literal::Boolean(lhs | rhs),
            _ => return None,
        })
    }

    pub fn b_xor(&self, rhs: &Self) -> Option<Self> {
        Some(match (self, rhs) {
            (Literal::Integer(lhs), Literal::Integer(rhs)) => Literal::Integer(lhs ^ rhs),
            (Literal::Boolean(lhs), Literal::Boolean(rhs)) => Literal::Boolean(lhs ^ rhs),
            _ => return None,
        })
    }

    pub fn l_and(&self, rhs: &Self) -> Option<Self> {
        Some(match (self, rhs) {
            (Literal::Boolean(lhs), Literal::Boolean(rhs)) => Literal::Boolean(*lhs && *rhs),
            _ => return None,
        })
    }

    pub fn l_or(&self, rhs: &Self) -> Option<Self> {
        Some(match (self, rhs) {
            (Literal::Boolean(lhs), Literal::Boolean(rhs)) => Literal::Boolean(*lhs || *rhs),
            _ => return None,
        })
    }

    pub fn mul(&self, rhs: &Self) -> Option<Self> {
        Some(match (self, rhs) {
            (Literal::FloatingPoint(lhs), Literal::FloatingPoint(rhs)) => {
                Literal::FloatingPoint(lhs * rhs)
            }
            (Literal::Integer(lhs), Literal::Integer(rhs)) => Literal::Integer(lhs * rhs),
            _ => return None,
        })
    }

    pub fn div(&self, rhs: &Self) -> Option<Self> {
        Some(match (self, rhs) {
            (Literal::FloatingPoint(lhs), Literal::FloatingPoint(rhs)) => {
                Literal::FloatingPoint(lhs / rhs)
            }
            (Literal::Integer(lhs), Literal::Integer(rhs)) => Literal::Integer(lhs / rhs),
            _ => return None,
        })
    }

    pub fn add(&self, rhs: &Self) -> Option<Self> {
        Some(match (self, rhs) {
            (Literal::FloatingPoint(lhs), Literal::FloatingPoint(rhs)) => {
                Literal::FloatingPoint(lhs + rhs)
            }
            (Literal::Integer(lhs), Literal::Integer(rhs)) => Literal::Integer(lhs + rhs),
            (Literal::String(lhs), Literal::String(rhs)) => Literal::String(lhs.clone() + &rhs),
            _ => return None,
        })
    }

    pub fn sub(&self, rhs: &Self) -> Option<Self> {
        Some(match (self, rhs) {
            (Literal::FloatingPoint(lhs), Literal::FloatingPoint(rhs)) => {
                Literal::FloatingPoint(lhs - rhs)
            }
            (Literal::Integer(lhs), Literal::Integer(rhs)) => Literal::Integer(lhs - rhs),
            _ => return None,
        })
    }

    pub fn ordering(&self, rhs: &Self) -> Option<Ordering> {
        match (self, rhs) {
            (Literal::String(lhs), Literal::String(rhs)) if lhs == rhs => Some(Ordering::Equal),
            (Literal::Character(lhs), Literal::Character(rhs)) => lhs.partial_cmp(rhs),
            (Literal::Integer(lhs), Literal::Integer(rhs)) => lhs.partial_cmp(rhs),
            (Literal::FloatingPoint(lhs), Literal::FloatingPoint(rhs)) => lhs.partial_cmp(rhs),
            (Literal::Boolean(lhs), Literal::Boolean(rhs)) => lhs.partial_cmp(rhs),
            (Literal::Nil, Literal::Nil) => Some(Ordering::Equal),
            _ => None,
        }
    }

    pub fn eq(&self, rhs: &Self) -> Option<bool> {
        self.ordering(rhs).map(|o| o.is_eq())
    }

    pub fn ne(&self, rhs: &Self) -> Option<bool> {
        self.ordering(rhs).map(|o| o.is_ne())
    }

    pub fn lt(&self, rhs: &Self) -> Option<bool> {
        self.ordering(rhs).map(|o| o.is_lt())
    }

    pub fn gt(&self, rhs: &Self) -> Option<bool> {
        self.ordering(rhs).map(|o| o.is_gt())
    }

    pub fn le(&self, rhs: &Self) -> Option<bool> {
        self.ordering(rhs).map(|o| o.is_le())
    }

    pub fn ge(&self, rhs: &Self) -> Option<bool> {
        self.ordering(rhs).map(|o| o.is_ge())
    }

    pub fn cast(&self, cast_to: &Type) -> Option<Self> {
        match cast_to {
            Type::String => match self {
                Literal::String(s) => Some(Literal::String(s.clone())),
                Literal::Character(c) => Some(Literal::String(format!("{c}"))),
                Literal::Integer(i) => Some(Literal::String(format!("{i}"))),
                Literal::FloatingPoint(f) => Some(Literal::String(format!("{f}"))),
                Literal::Boolean(b) => Some(Literal::String(format!("{b}"))),
                Literal::Nil => Some(Literal::String("nil".into())),
            },
            Type::Character => match self {
                Literal::Character(c) => Some(Literal::Character(c.clone())),
                Literal::Integer(i) => Some(Literal::Character(
                    i.to_u32().and_then(|u| char::from_u32(u))?,
                )),
                _ => None,
            },
            Type::Integer => match self {
                Literal::Character(c) => Some(Literal::Integer((*c as u32).into())),
                Literal::Boolean(b) => Some(Literal::Integer((*b as u8).into())),
                Literal::Nil => Some(Literal::Integer(0.into())),
                Literal::FloatingPoint(f) => {
                    if f.is_integer() {
                        let (integer, _) = f.as_bigint_and_exponent();
                        Some(Literal::Integer(integer))
                    } else {
                        None
                    }
                }
                _ => {
                    // TODO?
                    None
                }
            },
            Type::UnsignedInteger => {
                // TODO?
                None
            }
            Type::Boolean => match self {
                Literal::String(s) => Some(Literal::Boolean(s.len() > 0)),
                Literal::Character(_) => Some(Literal::Boolean(true)),
                Literal::Integer(i) => Some(Literal::Boolean(!i.is_zero())),
                Literal::FloatingPoint(f) => Some(Literal::Boolean(!f.is_zero())),
                Literal::Boolean(b) => Some(Literal::Boolean(*b)),
                Literal::Nil => Some(Literal::Boolean(false)),
            },
            Type::FloatingPoint => match self {
                Literal::Nil => Some(Literal::FloatingPoint(0.into())),
                Literal::Integer(i) => Some(Literal::FloatingPoint(i.clone().into())),
                _ => None,
            },
            Type::Custom(_) | Type::List(_) => None,
        }
    }
}

impl Parse<LiteralError> for Literal {
    fn parse(
        context: &mut ParseContext,
    ) -> ParseResult<Literal, LiteralError> {
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
    
                    Literal::String(parsed_string)
                }

                Token::LitChar(character) => {
                    if character.len() == 0 {
                        return Err(LiteralError::EmptyCharacterLiteral.to_parse_error(span.clone()));
                    } else {
                        let mut escaper = EscapeToken::lexer(&character);
    
                        match (escaper.next(), escaper.next()) {
                            (None, None) | (Some(_), Some(_)) => {
                                return Err(LiteralError::InvalidCharacter.to_parse_error(span.clone()));
                            }
                            (Some(token), None) => match token.char() {
                                Some(character) => Literal::Character(character),
                                None => {
                                    return Err(LiteralError::InvalidCharacter.to_parse_error(span.clone()));
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
                    Literal::Integer(integer)
                }

                Token::LitBool(boolean) => Literal::Boolean(*boolean),

                Token::LitFloat(float) => {
                    let float_string: String = float.chars().filter(|c| *c != '_').collect();
                    let float = match BigDecimal::from_str(&float_string) {
                        Ok(f) => f,
                        Err(e) => {
                            return Err(e.to_parse_error(span.clone()).convert());
                        }
                    };
                    Literal::FloatingPoint(float)
                }
    
                Token::LitNil => Literal::Nil,
                _ => {
                    return Err(ParseError::unexpected_token(span.clone()));
                }
            },
            None => return Err(ParseError::EndOfFile),
        };

        Ok(literal)
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
