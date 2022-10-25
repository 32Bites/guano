use std::ops::Range;

use guano_lexer::{Span, Token};
use thiserror::Error;

use crate::parser::{
    error::{ParseError, ParseResult, ToParseError, ToParseResult},
    Parse, ParseContext, TokenStream,
};

use super::{Statement, StatementError};

#[derive(Debug, Clone)]
pub struct Block {
    pub items: Vec<BlockItem>,
}

impl Parse<BlockError> for Block {
    fn parse(
        context: &mut ParseContext,
    ) -> ParseResult<Block, BlockError> {
        match context.stream.read_token::<1>()[0] {
            Some(Token::OpenBrace) => {
                let mut items: Vec<BlockItem> = vec![];

                loop {
                    let item = match &context.stream.peek_token::<1>()[0] {
                        None | Some(Token::CloseBrace) => {
                            context.stream.reset_peek();
                            break;
                        }

                        Some(token) => match token {
                            Token::OpenBrace => {
                                context.stream.reset_peek();
                                BlockItem::Block(Block::parse(context)?)
                            }

                            _ => {
                                context.stream.reset_peek();
                                BlockItem::Statement(Statement::parse(context).to_parse_result()?)
                            }
                        },
                    };

                    items.push(item);
                }

                if let Some(Token::CloseBrace) = context.stream.read_token::<1>()[0] {
                    Ok(Block { items })
                } else {
                    Err(BlockError::MissingClose.to_parse_error(None))
                }
            }
            Some(_) => Err(BlockError::InvalidStart.to_parse_error(None)),
            None => {
                context.stream.reset_peek();

                Err(ParseError::EndOfFile)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum BlockItem {
    Statement(Statement),
    Block(Block),
}

#[derive(Debug, Error)]
pub enum BlockError {
    #[error("{0}")]
    StatementError(#[from] StatementError),
    #[error("invalid start to code block")]
    InvalidStart,
    #[error("missing closing brace")]
    MissingClose,
}

#[cfg(test)]
mod tests {
    use crate::parser::Parse;

    use super::Block;

    #[test]
    fn test_block() {
        let source = r#"
        {
            let hello = "Hi!";
            {
                {
                    let muuuuut = 1;
                }
            }
            ;;;;;;;;
            {
                "";
            }
        }
        "#;
        // let mut parser = <Parser>::from_source(source, true);

        //println!("{:#?}", Block::parse(&mut parser));
    }
}
