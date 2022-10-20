use guano_lexer::{Span, Token};
use thiserror::Error;

use crate::{
    convert_result_impl,
    parser::{
        expression::{
            literal::Literal,
            Expression, ExpressionError,
        },
        identifier::{Identifier, IdentifierError},
        typing::{Type, TypeError},
        ConvertResult, Parse, Parser,
    },
};

#[derive(Debug, Clone)]
pub enum Mutability {
    Mutable,
    Immutable,
}

impl<I: Iterator<Item = (Token, Span)>> Parse<I, VariableError> for Mutability {
    fn parse(parser: &mut Parser<I>) -> Result<Self, Option<VariableError>> {
        let mutability =
            if let Some(token @ (Token::KeyLet | Token::KeyVar)) = &parser.peek_token::<1>()[0] {
                match token {
                    Token::KeyLet => Mutability::Immutable,
                    Token::KeyVar => Mutability::Mutable,
                    _ => unreachable!(),
                }
            } else {
                parser.reset_peek();
                return Err(None);
            };

        parser.read::<1>();

        Ok(mutability)
    }
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub identifier: Identifier,
    pub mutability: Mutability,
    pub value_type: Option<Type>,
    pub value: Expression,
}

impl<I: Iterator<Item = (Token, Span)>> Parse<I, VariableError> for Variable {
    fn parse(parser: &mut Parser<I>) -> Result<Variable, Option<VariableError>> {
        let mutability = Mutability::parse(parser)?;
        let identifier = Identifier::parse(parser).convert_result()?;

        let mut value_type = if let Some(Token::Colon) = parser.peek_token::<1>()[0] {
            parser.read::<1>();

            Some(Type::parse(parser).convert_result()?)
        } else {
            parser.reset_peek();
            None
        };

        let value = if let Some(Token::Equals) = parser.peek_token::<1>()[0] {
            parser.read::<1>();
            Expression::parse(parser).convert_result()?
        } else {
            parser.reset_peek();
            Literal::Nil.to_expression()
        };

        if let Some(value_type) = &value_type {
            if let Some(actual_type) = value.get_type() {
                if value_type.clone() != actual_type {
                    return Err(Some(VariableError::TypeMismatch));
                }
            }
        } else {
            value_type = value.get_type()
        }

        if let Some(Token::Semicolon) = parser.peek_token::<1>()[0] {
            parser.read::<1>();
            
            Ok(Variable {
                identifier,
                mutability,
                value_type,
                value,
            })
        } else {
            parser.reset_peek();
            Err(Some(VariableError::ExpectedSemicolon))
        }
    }
}

#[derive(Debug, Error)]
pub enum VariableError {
    #[error("{0}")]
    InvalidType(#[from] TypeError),
    #[error("{0}")]
    InvalidExpression(#[from] ExpressionError),
    #[error("{0}")]
    InvalidIdentifier(#[from] IdentifierError),
    #[error("value type of expression does not match the specified type")]
    TypeMismatch,
    #[error("no deliminating semicolon was found")]
    ExpectedSemicolon,
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

                                    let mut parser =
                                        <Parser>::from_source(statement.as_str(), true);

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
