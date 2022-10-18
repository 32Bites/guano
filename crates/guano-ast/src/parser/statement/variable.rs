use guano_lexer::{Span, Token};
use thiserror::Error;

use crate::{parser::{
    expression::{
        literal::Literal,
        parser::{Expression, ExpressionError},
    },
    typing::{Type, TypeError},
    ConvertResult, Parse, Parser,
}, convert_result_impl};

#[derive(Debug, Clone)]
pub enum Mutability {
    Mutable,
    Immutable,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub mutability: Mutability,
    pub value_type: Option<Type>,
    pub value: Expression,
}

impl<I: Iterator<Item = (Token, Span)> + std::fmt::Debug> Parse<I, VariableError> for Variable {
    fn parse(parser: &mut Parser<I>) -> Result<Variable, Option<VariableError>> {
        if let Some((token @ (Token::KeyLet | Token::KeyVar), _)) = parser.lexer.peek() {
            let mutability = match token {
                Token::KeyLet => Mutability::Immutable,
                Token::KeyVar => Mutability::Mutable,
                _ => unreachable!(),
            };
            parser.lexer.next();

            if let Some((Token::Identifier(name), _)) = parser.lexer.next() {
                let mut value_type = if let Some((Token::Colon, _)) = parser.lexer.peek() {
                    parser.lexer.next();
                    Some(Type::parse(parser).convert_result()?)
                } else {
                    parser.lexer.reset_peek();
                    None
                };

                let value = if let Some((Token::Equals, _)) = parser.lexer.peek() {
                    parser.lexer.next();
                    Expression::parse(parser).convert_result()?
                } else {
                    parser.lexer.reset_peek();
                    Literal::Nil.to_expression()
                };

                if let Some(value_type) = &value_type {
                    if let Some(actual_type) = value.get_type() {
                        if value_type.clone() != actual_type {
                            return Err(None);
                        }
                    }
                } else {
                    value_type = value.get_type()
                }

                if let Some((Token::Semicolon, _)) = parser.lexer.next() {
                    return Ok(Variable {
                        name,
                        mutability,
                        value_type,
                        value,
                    });
                }
            }
        } else {
            parser.lexer.reset_peek();
        }

        Err(None)
    }
}

#[derive(Debug, Error)]
pub enum VariableError {
    #[error("{0}")]
    InvalidType(#[from] TypeError),
    #[error("{0}")]
    InvalidExpression(#[from] ExpressionError),
}

convert_result_impl!(VariableError);

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::parser::{Parse, Parser};

    use super::Variable;

    #[test]
    fn test_var() {
        let mutabilities = ["", "var", "let"];
        let names = ["", "variable", "name", "ree"];
        let colons = ["", ":"];
        let type_names = ["", "uint", "int", "string", "float", "char"];
        let equals = ["", "="];
        let values = [
            "",
            "0",
            "1",
            "-1",
            "\"String\"",
            "4.0",
            "-4.0",
            "'c'",
            "nil",
        ];
        let semicolons = ["", ";"];

        for mutability in mutabilities {
            for name in names {
                for colon in colons {
                    for type_name in type_names {
                        for equal in equals {
                            for value in values {
                                for semicolon in semicolons {
                                    let start = [mutability, name, colon, type_name, equal, value]
                                        .iter()
                                        .filter(|s| !s.is_empty())
                                        .join(" ");
                                    let statement = format!("{start}{semicolon}");

                                    let mut parser = <Parser>::from_source(statement.as_str());

                                    if let Ok(variable) = Variable::parse(&mut parser) {
                                        println!("Success: {statement}");
                                        println!("Value: {variable:#?}");
                                    } else {
                                        //println!("Failure: {statement}");
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
