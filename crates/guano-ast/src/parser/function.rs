use guano_lexer::Token;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{
    block::{Block, BlockError},
    error::{ParseError, ParseResult, ToParseResult},
    identifier::{Identifier, IdentifierError},
    token_stream::{MergeSpan, Span, Spannable, Spanned, ToSpanned},
    typing::{Type, TypeError},
    Parse, ParseContext,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub identifier: Identifier,
    pub return_type: Option<Type>,
    pub arguments: Option<Spanned<Vec<Argument>>>,
    pub block: Block,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Argument {
    pub identifier: Identifier,
    pub associated_type: Type,
    pub span: Span,
}

impl Parse<FunctionError> for Function {
    fn parse(context: &mut ParseContext) -> ParseResult<Function, FunctionError> {
        let mut final_span = match &context.stream.read::<1>()[0] {
            Some((Token::KeyFun, span)) => span.clone(),
            Some((_, span)) => return Err(ParseError::unexpected_token(span.clone())),
            None => return Err(ParseError::EndOfFile),
        };

        let identifier = Identifier::parse(context).to_parse_result()?;

        final_span = final_span.merge(&identifier.span);

        let return_type = if let Some((Token::Colon, span)) = &context.stream.peek::<1>()[0] {
            final_span = final_span.merge(span);
            context.stream.read::<1>();

            let return_type = Type::parse(context).to_parse_result()?;
            final_span = final_span.merge(&return_type.span);

            Some(return_type)
        } else {
            context.stream.reset_peek();
            None // No return type
        };

        let arguments = Function::arguments(context).to_parse_result()?;

        if let Some(arguments) = &arguments {
            final_span = final_span.merge(&arguments.span);
        }

        let block = Block::parse(context).to_parse_result()?;

        final_span = final_span.merge(&block.span);

        Ok(Function {
            identifier,
            return_type,
            arguments,
            block,
            span: final_span,
        })
    }
}

impl Spannable for Function {
    fn get_span(&self) -> Span {
        self.span.clone()
    }
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fun {}", self.identifier)?;
        if let Some(return_type) = &self.return_type {
            write!(f, ": {return_type}")?;
        }
        f.write_str(" ")?;
        if let Some(Spanned {
            value: arguments,
            span: _,
        }) = &self.arguments
        {
            write!(
                f,
                "@ {} ",
                arguments
                    .iter()
                    .map(
                        |Argument {
                             identifier,
                             associated_type,
                             span: _,
                         }| format!("{identifier}: {associated_type}")
                    )
                    .join(", ")
            )?
        }

        self.block.fmt(f)
    }
}

impl Function {
    fn arguments(
        context: &mut ParseContext,
    ) -> ParseResult<Option<Spanned<Vec<Argument>>>, ArgumentError> {
        if let Some((Token::Asperand, span)) = &context.stream.peek::<1>()[0] {
            context.stream.read::<1>();
            let mut arguments = vec![];
            let mut final_span = span.clone();

            loop {
                let argument = Function::argument(context)?;
                final_span = final_span.merge(&argument.span);
                arguments.push(argument);

                if let Some((Token::Comma, span)) = &context.stream.peek::<1>()[0] {
                    final_span = final_span.merge(span);
                    context.stream.read::<1>();
                } else {
                    context.stream.reset_peek();
                    break;
                }
            }

            Ok(Some(arguments.to_spanned(final_span)))
        } else {
            context.stream.reset_peek();
            Ok(None)
        }
    }

    fn argument(context: &mut ParseContext) -> ParseResult<Argument, ArgumentError> {
        let identifier = Identifier::parse(context).to_parse_result()?;
        match &context.stream.read::<1>()[0] {
            Some((token, span)) => match token {
                Token::Colon => {
                    let associated_type = Type::parse(context).to_parse_result()?;
                    let span = identifier.span.merge(span).merge(&associated_type.span);

                    Ok(Argument {
                        identifier,
                        associated_type,
                        span,
                    })
                }

                _ => Err(ParseError::unexpected_token(span.clone())),
            },
            None => Err(ParseError::EndOfFile),
        }
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
