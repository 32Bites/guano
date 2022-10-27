use guano_lexer::Token;
use serde::{Serialize, Deserialize};

use crate::parser::{
    block::Block,
    error::{ParseError, ParseResult, ToParseResult},
    expression::Expression,
    Parse, ParseContext,
};

use super::StatementError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhileLoop {
    pub condition: Expression,
    pub block: Block,
}

impl Parse<StatementError> for WhileLoop {
    fn parse(context: &mut ParseContext) -> ParseResult<Self, StatementError> {
        match &context.stream.read::<1>()[0] {
            Some((token, span)) => match token {
                Token::KeyWhile => {
                    let condition = Expression::parse(context).to_parse_result()?;
                    let block = Block::parse(context).to_parse_result()?;
                    Ok(WhileLoop { condition, block })
                }
                _ => Err(ParseError::unexpected_token(span.clone())),
            },
            None => Err(ParseError::EndOfFile),
        }
    }
}

impl std::fmt::Display for WhileLoop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "while {} {}", self.condition, self.block)
    }
}
