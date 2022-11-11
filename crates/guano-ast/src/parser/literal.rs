use std::str::FromStr;

use bigdecimal::{BigDecimal, Num};
use num::{BigInt, BigUint};
use owning_ref::RcRef;
use pest::iterators::Pair;
use thiserror::Error;

use super::{expression::{Expression, ExpressionError}, parser::Rule};

#[derive(Debug, Clone)]
pub enum Literal {
    Nil,
    Boolean(bool),
    Integer(Integer),
    FloatingPoint(BigDecimal),
    Character(char),
    String(String),
    List(Vec<Expression>),
    Tuple(Vec<Expression>),
    Format {
        format_string: String,
        format_with: Box<Expression>,
    },
}

impl Literal {
    pub fn parse(pair: Pair<'_, Rule>, input: RcRef<str>) -> Result<Self, LiteralError> {
        Ok(match pair.as_rule() {
            Rule::literal
            | Rule::number_literal
            | Rule::text_literal
            | Rule::collection_literal => return Self::parse(pair.into_inner().next().unwrap(), input),
            Rule::nil_literal => Literal::Nil,
            Rule::boolean_literal => {
                if pair.as_str() == "true" {
                    Literal::Boolean(true)
                } else {
                    Literal::Boolean(false)
                }
            }
            Rule::integer => Literal::Integer(Integer::from(pair)),
            Rule::string_literal => {
                let value = parse_string_literal(pair)?;
                Literal::String(value)
            }
            Rule::character_literal => {
                let value = text_inner_to_char(pair.into_inner().next().unwrap())?;
                Literal::Character(value)
            },
            Rule::format_string => {
                let mut inner = pair.into_inner();
                let format_string = parse_string_literal(inner.next().unwrap())?;
                let expression = Expression::parse(inner.next().unwrap().into_inner(), input).map_err(|e| Box::new(e))?;

                Literal::Format { format_string, format_with: Box::new(expression) }
            },
            Rule::list_literal => {
                let mut values = vec![];
                match pair.into_inner().next() {
                    Some(list_inner) => {
                        for value in list_inner.into_inner() {
                            let expression = Expression::parse(value.into_inner(), input.clone()).map_err(|e| Box::new(e))?;
                            values.push(expression)
                        }
                    },
                    None => {},
                }

                Literal::List(values)
            },
            Rule::tuple_literal => {
                let mut values = vec![];
                match pair.into_inner().next() {
                    Some(list_inner) => {
                        for value in list_inner.into_inner() {
                            let expression = Expression::parse(value.into_inner(), input.clone()).map_err(|e| Box::new(e))?;
                            values.push(expression)
                        }
                    },
                    None => {},
                }

                Literal::Tuple(values)
            },
            Rule::floating_point => {
                let cleaned = pair.as_str().replace("_", "");
                let parsed = BigDecimal::from_str(&cleaned).unwrap();

                Literal::FloatingPoint(parsed)
            }
            _ => todo!(),
        })
    }
}

#[derive(Debug, Clone, Error)]
pub enum LiteralError {
    #[error("{0}")]
    EscapeError(#[from] EscapeError),
    #[error("{0}")]
    ExpressionError(#[from] Box<ExpressionError>)
}

#[derive(Debug, Clone, Error)]
pub enum EscapeError {
    #[error("invalid ascii")]
    InvalidAscii,
    #[error("invalid unicode character code")]
    InvalidUnicode,
}

pub(crate) fn parse_string_literal(pair: Pair<Rule>) -> Result<String, EscapeError> {
    let mut value = "".to_string();

    for pair in pair.into_inner() {
        value.push(text_inner_to_char(pair)?)
    }

    Ok(value)
}

fn text_inner_to_char(pair: Pair<Rule>) -> Result<char, EscapeError> {
    Ok(match pair.clone().into_inner().next() {
        Some(inner) => match inner.as_rule() {
            Rule::text_inner => return text_inner_to_char(inner),
            Rule::single_escape => match &inner.as_str()[1..] {
                "t" => '\t',
                "r" => '\r',
                "n" => '\n',
                "0" => '\0',
                "\\" => '\\',
                "'" => '\'',
                "\"" => '"',
                _ => unreachable!(),
            },
            Rule::ascii_escape => {
                let hex = &pair.as_str()[2..];
                let char_code = u8::from_str_radix(hex, 16).unwrap();

                if char_code <= 0x7F {
                    char_code as char
                } else {
                    return Err(EscapeError::InvalidAscii);
                }
            }
            Rule::unicode_escape => {
                let hex = &inner.as_str()[2..];
                let char_code = u32::from_str_radix(hex, 16).unwrap();

                char::from_u32(char_code).ok_or(EscapeError::InvalidUnicode)?
            }
            _ => unreachable!(),
        },
        None => pair.as_str().chars().next().unwrap(),
    })
}

#[derive(Debug, Clone)]
pub enum Integer {
    Decimal(BigInt),
    Binary(BigUint),
    Hexadecimal(BigUint),
}

impl From<Pair<'_, Rule>> for Integer {
    fn from(pair: Pair<'_, Rule>) -> Self {
        match pair.as_rule() {
            Rule::integer => pair.into_inner().next().unwrap().into(),
            Rule::binary_integer => {
                let cleaned: String = pair.as_str().replace("_", "");
                let parsed = BigUint::from_str_radix(cleaned.trim_start_matches("0b"), 2).unwrap();

                Integer::Binary(parsed)
            }
            Rule::hexadecial_integer => {
                let cleaned: String = pair.as_str().replace("_", "");
                let parsed = BigUint::from_str_radix(cleaned.trim_start_matches("0x"), 16).unwrap();

                Integer::Hexadecimal(parsed)
            }
            Rule::decimal_integer => {
                let cleaned: String = pair.as_str().replace("_", "");
                let parsed = BigInt::from_str_radix(&cleaned, 10).unwrap();

                Integer::Decimal(parsed)
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser;

    use crate::parser::{InternalParser, Rule};

    #[test]
    fn test_literal() {
        let expressions: Vec<_> = [
            "true",
            "false",
            "nil",
            "100",
            "0xFF",
            "0b10001",
            "100.5",
            " 'H' ",
            r#" '\x00' "#,
            r#" '\u0000' "#,
            r#" '\U00000000' "#,
            r#"   " Hi\" \U00000000 \u0000 \x00 \0 "   "#,
            "[]",
            "[1]",
            "[1,]",
            "[1, 1, 'H']",
            "()",
            "(1)",
            "(1,)",
            "(1, 2)",
            "(1, 2, 3)",
            "(1, 2,)"
        ].into_iter().map(|s| s.trim()).collect();

        for expression in expressions {
            if let Ok(res) = InternalParser::parse(Rule::literal, expression) {
                println!("Success: {expression}");
                for _pair in res {
/*                     let ty = Literal::try_from(pair);
                    println!("\t{ty:?}"); */
                }
            } else {
                println!("Error: {expression}");
            }

            println!("-----------------------------");
        }
    }
}
