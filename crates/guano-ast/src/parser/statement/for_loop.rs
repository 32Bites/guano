use guano_lexer::Token;
use serde::{Serialize, Deserialize};

use crate::parser::{
    block::Block,
    error::{ParseError, ParseResult, ToParseResult},
    expression::Expression,
    identifier::Identifier,
    Parse, ParseContext,
};

use super::StatementError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForLoop {
    pub identifier: Identifier,
    pub iterator: Expression,
    pub block: Block,
}

impl Parse<StatementError> for ForLoop {
    fn parse(context: &mut ParseContext) -> ParseResult<Self, StatementError> {
        match &context.stream.read::<1>()[0] {
            Some((token, span)) => match token {
                Token::KeyFor => {
                    let identifier = Identifier::parse(context).to_parse_result()?;

                    match &context.stream.read::<1>()[0] {
                        Some((token, span)) => match token {
                            Token::KeyIn => {}
                            _ => return Err(ParseError::unexpected_token(span.clone())),
                        },
                        None => return Err(ParseError::EndOfFile),
                    }

                    let iterator = Expression::parse(context).to_parse_result()?;
                    let block = Block::parse(context).to_parse_result()?;

                    Ok(ForLoop {
                        identifier,
                        iterator,
                        block,
                    })
                }
                _ => Err(ParseError::unexpected_token(span.clone())),
            },
            None => Err(ParseError::EndOfFile),
        }
    }
}

impl std::fmt::Display for ForLoop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "for {} in {} {}",
            self.identifier, self.iterator, self.block
        )
    }
}
