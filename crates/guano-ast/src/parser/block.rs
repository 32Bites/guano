use guano_lexer::Token;
use indenter::indented;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use thiserror::Error;

use crate::parser::{
    error::{ParseError, ParseResult, ToParseError},
    Parse, ParseContext,
};

use super::{
    statement::{Statement, StatementError},
    token_stream::{MergeSpan, Span, Spannable},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub items: Vec<BlockItem>,
    pub span: Span,
}

impl Spannable for Block {
    fn get_span(&self) -> Span {
        self.span.clone()
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("{\n")?;
        for item in &self.items {
            writeln!(indented(f), "{item}")?;
        }

        f.write_str("}")
    }
}

impl Parse<BlockError> for Block {
    fn parse(context: &mut ParseContext) -> ParseResult<Block, BlockError> {
        let mut final_span = match &context.stream.read::<1>()[0] {
            Some((token, span)) => match token {
                Token::OpenBrace => span.clone(),
                _ => return Err(ParseError::unexpected_token(span.clone())),
            },

            None => return Err(ParseError::EndOfFile),
        };

        let mut items: Vec<BlockItem> = vec![];

        loop {
            let item = match &context.stream.peek_token::<1>()[0] {
                None | Some(Token::CloseBrace) => {
                    context.stream.reset_peek();
                    break;
                }

                Some(token) => match token {
                    Token::OpenBrace => {
                        let block = Block::parse(context)?;
                        final_span = final_span.merge(&block.span);
                        BlockItem::Block(block)
                    }

                    _ => {
                        context.stream.reset_peek();
                        let statement =
                            match Statement::parse(context).map_err(|e| e.convert_boxed())? {
                                Ok(statement) => statement,
                                Err(span) => {
                                    final_span = final_span.merge(&span);
                                    continue;
                                }
                            };
                        final_span = final_span.merge(&statement.span);

                        BlockItem::Statement(statement)
                    }
                },
            };

            items.push(item);
        }

        match &context.stream.read::<1>()[0] {
            Some((Token::CloseBrace, span)) => {
                final_span = final_span.merge(span);
                Ok(Block {
                    items,
                    span: final_span,
                })
            }
            Some((_, span)) => Err(BlockError::MissingClose.to_parse_error(span.clone())),
            None => Err(ParseError::EndOfFile),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockItem {
    Statement(Statement),
    Block(Block),
}

impl std::fmt::Display for BlockItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockItem::Statement(s) => s.fmt(f),
            BlockItem::Block(b) => b.fmt(f),
        }
    }
}

#[derive(Debug, Error)]
pub enum BlockError {
    #[error("{0}")]
    StatementError(#[from] Box<StatementError>),
    #[error("missing closing brace")]
    MissingClose,
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    use super::Block;

    #[test]
    fn test_block() {
        let source = r#"
        {
            let hello = "Hi!";
            {
                {
                    let muuuuut = 1;
                    return;
                    return "Hi";
                }
            }
            ;;;;;;;;
            {
                "";}}
        "#;

        let mut parser = Parser::new(false);
        let (_, result) = parser.parse_file::<Block, _, _>("", source);
        if let Ok(block) = result {
            println!("{block}")
        }
        // let mut parser = <Parser>::from_source(source, true);

        //println!("{:#?}", Block::parse(&mut parser));
    }
}
