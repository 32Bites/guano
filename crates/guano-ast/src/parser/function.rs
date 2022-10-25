use guano_lexer::Token;
use indexmap::{indexmap, IndexMap};
use thiserror::Error;

use super::{
    error::{ParseError, ParseResult, ToParseResult},
    identifier::{Identifier, IdentifierError},
    statement::block::{Block, BlockError},
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
    fn parse(
        parser: &mut ParseContext,
    ) -> ParseResult<Function, FunctionError> {
        let name = Function::name(parser)?;
        let return_type = todo!(); //Typed::parse_backtrack(parser).ok();
        let arguments = Function::arguments(parser).to_parse_result()?;
        let block = Block::parse(parser).to_parse_result()?;

        Ok(Function {
            name,
            return_type,
            arguments,
            block,
        })
    }
}

/// I'm probably going to redo most parsers like this
/// where it is broken up into individual functions that parse
/// the various parts of a structure.
impl Function {
    fn name(
        parser: &mut ParseContext,
    ) -> ParseResult<Identifier, FunctionError> {
        match &parser.stream.read::<1>()[0] {
            Some((Token::KeyFun, _)) => Identifier::parse(parser).to_parse_result(),
            Some((_, span)) => Err(ParseError::unexpected_token(span.clone())),
            None => Err(ParseError::EndOfFile),
        }
    }

    fn return_type(
        parser: &mut ParseContext,
    ) -> ParseResult<Option<Type>, FunctionError> {
        if let Some(Token::Colon) = parser.stream.peek_token::<1>()[0] {
            parser.stream.read::<1>();

            Ok(Some(Type::parse(parser).to_parse_result()?))
        } else {
            parser.stream.reset_peek();
            Ok(None) // No return type
        }
    }

    fn arguments(
        parser: &mut ParseContext,
    ) -> ParseResult<IndexMap<Identifier, Type>, ArgumentError> {
        let mut arguments = indexmap! {};
        if let Some(Token::Asperand) = parser.stream.peek_token::<1>()[0] {
            parser.stream.read::<1>();

            loop {
                let (identifier, argument_type) = Function::argument(parser)?;
                arguments.insert(identifier, argument_type); // TODO: handle duplicates

                if let Some(Token::Comma) = parser.stream.peek_token::<1>()[0] {
                    parser.stream.read::<1>();
                } else {
                    parser.stream.reset_peek();
                    break;
                }
            }
        } else {
            parser.stream.reset_peek();
        }

        Ok(arguments)
    }

    fn argument(
        parser: &mut ParseContext,
    ) -> ParseResult<(Identifier, Type), ArgumentError> {
        Ok((
            Identifier::parse(parser).to_parse_result()?,
            todo!(), /* Typed::parse(parser).to_parse_result()? */
        ))
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
}
