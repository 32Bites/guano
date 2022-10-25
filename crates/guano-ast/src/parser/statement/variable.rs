use guano_lexer::Token;
use thiserror::Error;

use crate::parser::{
    error::{ParseError, ParseResult, ToParseError, ToParseResult},
    expression::{Expression, ExpressionError},
    identifier::{Identifier, IdentifierError},
    typing::{Type, TypeError},
    Parse, ParseContext,
};

#[derive(Debug, Clone)]
pub struct Variable {
    pub mutability: Mutability,
    pub identifier: Identifier,
    pub provided_type: Option<Type>,
    pub value: Option<Expression>,
}

impl Variable {
    fn parse_name(
        context: &mut ParseContext,
        mutability: &Mutability,
    ) -> ParseResult<Identifier, VariableError> {
        match &context.stream.peek::<1>()[0] {
            Some((Token::Identifier(_), _)) => Identifier::parse(context).to_parse_result(),
            Some((_, span)) => {
                context.stream.reset_peek();
                let mut test_context = context
                    .prepend([
                        (mutability.token(), None),
                        (Token::Identifier("TEST".into()), None),
                    ])
                    .ok_or(ParseError::unexpected_token(span.clone()))?;

                match Variable::parse(&mut test_context) {
                    Ok(_) => Err(VariableError::MissingName.to_parse_error(span.clone())),
                    Err(_) => Err(ParseError::unexpected_token(span.clone())),
                }
            }

            None => Err(ParseError::EndOfFile),
        }
    }
}

impl Parse<VariableError> for Variable {
    fn parse(context: &mut ParseContext) -> ParseResult<Variable, VariableError> {
        let mutability = Mutability::parse(context)?;
        let identifier = Variable::parse_name(context, &mutability)?;

        let provided_type = match context.stream.peek_token::<1>()[0] {
            Some(Token::Colon) => {
                context.stream.read::<1>();
                Some(Type::parse(context).to_parse_result()?)
            }
            _ => {
                context.stream.reset_peek();
                None
            }
        };

        let value = match context.stream.peek_token::<1>()[0] {
            Some(Token::Equals) => {
                context.stream.read::<1>();

                Some(Expression::parse(context).to_parse_result()?)
            }

            _ => {
                context.stream.reset_peek();
                None
            }
        };

        Ok(Variable {
            mutability,
            identifier,
            provided_type,
            value,
        })
    }
}

#[derive(Debug, Clone)]
pub enum Mutability {
    Mutable,
    Immutable,
}

impl Mutability {
    fn token(&self) -> Token {
        match self {
            Mutability::Mutable => Token::KeyVar,
            Mutability::Immutable => Token::KeyLet,
        }
    }
}

impl Parse<VariableError> for Mutability {
    fn parse(parser: &mut ParseContext) -> ParseResult<Mutability, VariableError> {
        let mutability = match &parser.stream.peek::<1>()[0] {
            Some((Token::KeyLet, _)) => Ok(Mutability::Immutable),
            Some((Token::KeyVar, _)) => Ok(Mutability::Mutable),
            t => {
                parser.stream.reset_peek();

                return match t {
                    Some((_, s)) => Err(VariableError::InvalidMutability.to_parse_error(s.clone())),
                    None => Err(ParseError::EndOfFile),
                };
            }
        };
        parser.stream.read::<1>();

        mutability
    }
}

#[derive(Debug, Error)]
pub enum VariableError {
    #[error("missing variable name")]
    MissingName,
    #[error("{0}")]
    InvalidType(#[from] TypeError),
    #[error("{0}")]
    InvalidExpression(#[from] ExpressionError),
    #[error("{0}")]
    InvalidIdentifier(#[from] IdentifierError),
    /*     #[error("value type of expression does not match the specified type")]
    TypeMismatch, */
    #[error("variable statements must start with `var` or `let`")]
    InvalidMutability,
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::parser::{error::ParseError, Parser};

    use super::Variable;

    #[test]
    fn test_var() {
        let mutabilities = ["", "var", "let"];
        let names = ["", "variable", "name", "ree"];
        let colons = ["", ":"];
        let type_names = ["", "uint", "int", "string", "float", "character"];
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
                                    let statement =
                                        [mutability, name, colon, type_name, equal, value]
                                            .iter()
                                            .filter(|s| !s.is_empty())
                                            .join(" ");
                                    let statement = statement + semicolon;

                                    let mut parser = Parser::new(false);
                                    let (_, result) =
                                        parser.parse_file::<Variable, _, _>("", &*statement);

                                    match result {
                                        Ok(_decl) => {
                                            // println!("👍 {statement}");
                                            // println!("{decl:#?}")
                                        }
                                        Err(parse_error) => match parse_error {
                                            ParseError::Spanned(Some(error), _) => match error {
                                                super::VariableError::MissingName => {
                                                    println!("Missing name in {statement:?}");
                                                }
                                                _ => {}
                                            },
                                            _ => {}
                                        },
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
