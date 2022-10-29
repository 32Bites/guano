use guano_lexer::Token;
use serde::{Deserialize, Serialize};

use crate::parser::{
    block::Block,
    error::{ParseError, ParseResult, ToParseResult},
    expression::Expression,
    identifier::Identifier,
    token_stream::{MergeSpan, Spanned, ToSpanned},
    Parse, ParseContext,
};

use super::StatementError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForLoop {
    pub identifier: Identifier,
    pub iterator: Expression,
    pub block: Block,
}

impl Parse<StatementError, Spanned<ForLoop>> for ForLoop {
    fn parse(context: &mut ParseContext) -> ParseResult<Spanned<ForLoop>, StatementError> {
        match &context.stream.read::<1>()[0] {
            Some((token, span)) => match token {
                Token::KeyFor => {
                    let mut final_span = span.clone();
                    let identifier = Identifier::parse(context).to_parse_result()?;
                    final_span = final_span.merge(&identifier.span);

                    match &context.stream.read::<1>()[0] {
                        Some((token, span)) => match token {
                            Token::KeyIn => {
                                final_span = final_span.merge(span);
                            }
                            _ => return Err(ParseError::unexpected_token(span.clone())),
                        },
                        None => return Err(ParseError::EndOfFile),
                    }

                    let iterator = Expression::parse(context).to_parse_result()?;
                    final_span = final_span.merge(&iterator.span);
                    let block = Block::parse(context).to_parse_result()?;
                    final_span = final_span.merge(&block.span);

                    Ok(ForLoop {
                        identifier,
                        iterator,
                        block,
                    }
                    .to_spanned(final_span))
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
