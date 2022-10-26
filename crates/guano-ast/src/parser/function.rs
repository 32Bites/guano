use guano_lexer::Token;
use indexmap::{indexmap, IndexMap};
use itertools::Itertools;
use thiserror::Error;

use super::{
    block::{Block, BlockError},
    error::{ParseError, ParseResult, ToParseError, ToParseResult},
    identifier::{Identifier, IdentifierError},
    typing::{Type, TypeError},
    Parse, ParseContext,
};

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Identifier,
    pub return_type: Option<Type>,
    pub arguments: IndexMap<Identifier, Type>,
    pub block: Block,
}

impl Parse<FunctionError> for Function {
    fn parse(context: &mut ParseContext) -> ParseResult<Function, FunctionError> {
        let name = Function::name(context)?;
        let return_type = Function::return_type(context)?;
        let arguments = Function::arguments(context).to_parse_result()?;
        let block = Block::parse(context).to_parse_result()?;

        Ok(Function {
            name,
            return_type,
            arguments,
            block,
        })
    }
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fun {}", self.name)?;
        if let Some(return_type) = &self.return_type {
            write!(f, ": {return_type}")?;
        }
        if self.arguments.len() > 0 {
            write!(
                f,
                " @ {}",
                self.arguments
                    .iter()
                    .map(|(i, t)| format!("{i}: {t}"))
                    .join(", ")
            )?
        }
        write!(f, " {}", self.block)
    }
}

impl Function {
    fn name(context: &mut ParseContext) -> ParseResult<Identifier, FunctionError> {
        match &context.stream.read::<1>()[0] {
            Some((Token::KeyFun, _)) => Identifier::parse(context).to_parse_result(),
            Some((_, span)) => Err(ParseError::unexpected_token(span.clone())),
            None => Err(ParseError::EndOfFile),
        }
    }

    fn return_type(context: &mut ParseContext) -> ParseResult<Option<Type>, FunctionError> {
        if let Some(Token::Colon) = context.stream.peek_token::<1>()[0] {
            context.stream.read::<1>();

            Ok(Some(Type::parse(context).to_parse_result()?))
        } else {
            context.stream.reset_peek();
            Ok(None) // No return type
        }
    }

    fn arguments(
        context: &mut ParseContext,
    ) -> ParseResult<IndexMap<Identifier, Type>, ArgumentError> {
        let mut arguments = indexmap! {};
        if let Some(Token::Asperand) = context.stream.peek_token::<1>()[0] {
            context.stream.read::<1>();

            loop {
                let (identifier, argument_type) = Function::argument(context)?;
                if arguments.insert(identifier, argument_type).is_some() {
                    return Err(ArgumentError::DuplicateArgument.to_parse_error(None));
                }

                if let Some(Token::Comma) = context.stream.peek_token::<1>()[0] {
                    context.stream.read::<1>();
                } else {
                    context.stream.reset_peek();
                    break;
                }
            }
        } else {
            context.stream.reset_peek();
        }

        Ok(arguments)
    }

    fn argument(context: &mut ParseContext) -> ParseResult<(Identifier, Type), ArgumentError> {
        let identifier = Identifier::parse(context).to_parse_result()?;
        let associated_type = match &context.stream.read::<1>()[0] {
            Some((token, span)) => match token {
                Token::Colon => Type::parse(context).to_parse_result()?,

                _ => return Err(ParseError::unexpected_token(span.clone())),
            },
            None => return Err(ParseError::EndOfFile),
        };
        Ok((identifier, associated_type))
    }
}

#[derive(Debug, Error)]
pub enum FunctionError {
    #[error("invalid function name")]
    InvalidName(#[from] IdentifierError),
    #[error("invalid return type")]
    TypeError(#[from] TypeError),
    #[error("invalid code block")]
    BlockError(#[from] BlockError),
    #[error("invalid argument")]
    ArgumentError(#[from] ArgumentError),
    #[error("unexpected start to function declaration")]
    UnexpectedStart,
}

#[derive(Debug, Error)]
pub enum ArgumentError {
    #[error("invalid argument name")]
    InvalidName(#[from] IdentifierError),
    #[error("invalid type")]
    InvalidType(#[from] TypeError),
    #[error("more than one argument with name")]
    DuplicateArgument,
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    use super::Function;

    #[test]
    fn test_function() {
        let source = r#"
        fun main @ args: []string {
            let name: string = "Noah";
            let first: float = 11.0;
            let second: uint = 6;
            let age: float = add(first, second);
            let new_age: float = add(first, add(second, second));
        
            personPrint(age, name);
        }
        "#;
        let mut parser = Parser::new(false);
        let (_, result) = parser.parse_file::<Function, _, _>("", source);
        match result {
            Ok(function) => println!("{function}"),
            Err(error) => {
                println!("Error: {error}");
                if let Some(span) = error.span() {
                    println!("Error String: {:#?}", &source[span])
                }
            }
        }
    }
}
