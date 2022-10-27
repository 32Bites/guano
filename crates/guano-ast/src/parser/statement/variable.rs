use guano_lexer::Token;
use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::parser::{
    error::{ParseError, ParseResult, ToParseError, ToParseResult},
    expression::Expression,
    identifier::Identifier,
    typing::{Type, TypeError},
    Parse, ParseContext,
};

use super::StatementError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    pub mutability: Mutability,
    pub name: Identifier,
    pub value_type: Option<Type>,
    pub value: Option<Expression>,
}

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_string = self
            .value_type
            .as_ref()
            .map_or("".to_string(), |t| format!(": {t}"));
        let value_string = self
            .value
            .as_ref()
            .map_or("".to_string(), |v| format!(" = {v}"));

        write!(
            f,
            "{} {}{type_string}{value_string}",
            self.mutability, self.name
        )
    }
}

impl Variable {
    fn parse_name(
        context: &mut ParseContext,
        mutability: &Mutability,
    ) -> ParseResult<Identifier, StatementError> {
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
                    Ok(_) => Err(VariableError::MissingName.to_parse_error(span.clone()))
                        .to_parse_result(),
                    Err(_) => Err(ParseError::unexpected_token(span.clone())),
                }
            }

            None => Err(ParseError::EndOfFile),
        }
    }
}

impl Parse<StatementError> for Variable {
    fn parse(context: &mut ParseContext) -> ParseResult<Variable, StatementError> {
        let mutability = Mutability::parse(context)?;
        let identifier = Variable::parse_name(context, &mutability)?;

        let provided_type = match context.stream.peek_token::<1>()[0] {
            Some(Token::Colon) => {
                context.stream.read::<1>();
                Some(
                    Type::parse(context)
                        .map_err(|e| e.convert::<VariableError>())
                        .to_parse_result()?,
                )
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
            name: identifier,
            value_type: provided_type,
            value,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mutability {
    Mutable,
    Immutable,
}

impl std::fmt::Display for Mutability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Mutability::Mutable => "var",
            Mutability::Immutable => "let",
        })
    }
}

impl Mutability {
    fn token(&self) -> Token {
        match self {
            Mutability::Mutable => Token::KeyVar,
            Mutability::Immutable => Token::KeyLet,
        }
    }
}

impl Parse<StatementError> for Mutability {
    fn parse(parser: &mut ParseContext) -> ParseResult<Mutability, StatementError> {
        match &parser.stream.read::<1>()[0] {
            Some((Token::KeyLet, _)) => Ok(Mutability::Immutable),
            Some((Token::KeyVar, _)) => Ok(Mutability::Mutable),
            Some((_, span)) => Err(ParseError::unexpected_token(span.clone())),
            None => Err(ParseError::EndOfFile),
        }
    }
}

#[derive(Debug, Error)]
pub enum VariableError {
    #[error("missing variable name")]
    MissingName,
    #[error("{0}")]
    InvalidType(#[from] TypeError),
    /*     #[error("value type of expression does not match the specified type")]
    TypeMismatch, */
    #[error("variable statements must start with `var` or `let`")]
    InvalidMutability,
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::parser::{statement::Statement, Parser};

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
                                        parser.parse_file::<Statement, _, _>("", &*statement);

                                    match result {
                                        Ok(decl) => {
                                            println!("👍 {statement}");
                                            println!("{decl:#?}")
                                        }
                                        Err(_) => {}
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
