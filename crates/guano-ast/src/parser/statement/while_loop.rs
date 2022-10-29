use guano_lexer::Token;
use serde::{Deserialize, Serialize};

use crate::parser::{
    block::Block,
    error::{ParseError, ParseResult, ToParseResult},
    expression::Expression,
    token_stream::{MergeSpan, Spanned, ToSpanned},
    Parse, ParseContext,
};

use super::StatementError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhileLoop {
    pub condition: Expression,
    pub block: Block,
}

impl Parse<StatementError, Spanned<WhileLoop>> for WhileLoop {
    fn parse(context: &mut ParseContext) -> ParseResult<Spanned<WhileLoop>, StatementError> {
        match &context.stream.read::<1>()[0] {
            Some((token, span)) => match token {
                Token::KeyWhile => {
                    let condition = Expression::parse(context).to_parse_result()?;
                    let block = Block::parse(context).to_parse_result()?;

                    let span = span.merge(&condition.span).merge(&block.span);
                    Ok(WhileLoop { condition, block }.to_spanned(span))
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
